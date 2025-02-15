mod driver;
mod feature;

use super::{Page, Profile};
use driver::Driver;
use feature::Feature;
use gtk::{
    gio::Cancellable,
    glib::{Uri, UriFlags},
    prelude::{ActionExt, CancellableExt},
};
use std::{cell::Cell, rc::Rc};

/// Multi-protocol client API for tab `Item`
pub struct Client {
    cancellable: Cell<Cancellable>,
    driver: Rc<Driver>,
    page: Rc<Page>,
    profile: Rc<Profile>,
}

impl Client {
    // Constructors

    /// Create new `Self`
    pub fn init(profile: &Rc<Profile>, page: &Rc<Page>) -> Self {
        Self {
            cancellable: Cell::new(Cancellable::new()),
            driver: Rc::new(Driver::build(page)),
            page: page.clone(),
            profile: profile.clone(),
        }
    }

    // Actions

    /// Route tab item `request` to protocol driver
    /// * or `navigation` entry if the value not provided
    pub fn handle(&self, request: &str, is_snap_history: bool) {
        // Move focus out from navigation entry @TODO
        self.page.browser_action.escape.activate(None);

        // Initially disable find action
        self.page
            .window_action
            .find
            .simple_action
            .set_enabled(false);

        // Reset widgets
        self.page.search.unset();
        self.page.input.unset();
        self.page.set_title("Loading..");
        self.page.set_progress(0.1);

        if is_snap_history {
            snap_history(&self.page, None);
        }

        // try autocomplete scheme if the request match local filename
        if std::path::Path::new(&request).exists() {
            self.page
                .item_action
                .load
                .activate(Some(&format!("file://{request}")), is_snap_history)
        } else {
            // run async resolver to detect Uri, scheme-less host, or search query
            lookup(&self.profile, request, self.cancellable(), {
                let driver = self.driver.clone();
                let page = self.page.clone();
                move |feature, cancellable, result| {
                    match result {
                        // route by scheme
                        Ok(uri) => match uri.scheme().as_str() {
                            "file" => driver.file.handle(uri, feature, cancellable),
                            "gemini" | "titan" => driver.gemini.handle(uri, feature, cancellable),
                            scheme => {
                                // no scheme match driver, complete with failure message
                                let status = page.content.to_status_failure();
                                status.set_description(Some(&format!(
                                    "Scheme `{scheme}` yet not supported"
                                )));
                                page.set_title(&status.title());
                                page.set_progress(0.0);
                            }
                        },
                        // begin redirection to new address suggested
                        Err(uri) => page
                            .item_action
                            .load
                            .activate(Some(&uri.to_string()), false),
                    }
                }
            })
        }
    }

    /// Get new [Cancellable](https://docs.gtk.org/gio/class.Cancellable.html) by cancel previous one
    fn cancellable(&self) -> Cancellable {
        // Init new Cancellable
        let cancellable = Cancellable::new();

        // Replace by cancel previous operations
        let previous = self.cancellable.replace(cancellable.clone());
        if !previous.is_cancelled() {
            previous.cancel();
        }

        // Done
        cancellable
    }
}

/// Create request using async DNS resolver (slow method)
/// * return suggestion [Uri](https://docs.gtk.org/glib/struct.Uri.html) on failure (to handle as redirect)
fn lookup(
    profile: &Rc<Profile>,
    query: &str,
    cancellable: Cancellable,
    callback: impl FnOnce(Rc<Feature>, Cancellable, Result<Uri, Uri>) + 'static,
) {
    use gtk::{
        gio::{NetworkAddress, Resolver},
        prelude::{NetworkAddressExt, ResolverExt},
    };

    const DEFAULT_SCHEME: &str = "gemini";
    const DEFAULT_PORT: u16 = 1965;
    const TIMEOUT: u32 = 1000; // ms @TODO optional

    let (feature, query) = Feature::parse(query.trim());
    let feature = Rc::new(feature);

    match Uri::parse(query, UriFlags::NONE) {
        Ok(uri) => callback(feature, cancellable, Ok(uri)),
        Err(_) => {
            // try default scheme suggestion
            let suggestion = format!(
                "{DEFAULT_SCHEME}://{}",
                query
                    .strip_prefix(&format!("{DEFAULT_SCHEME}://"))
                    .unwrap_or(query)
            );

            let resolver = Resolver::default();
            resolver.set_timeout(TIMEOUT);

            match NetworkAddress::parse_uri(&suggestion, DEFAULT_PORT) {
                Ok(connectable) => resolver.lookup_by_name_async(
                    &connectable.hostname(),
                    Some(&cancellable.clone()),
                    {
                        let profile = profile.clone();
                        let query = query.to_owned();
                        move |resolve| {
                            callback(
                                feature,
                                cancellable,
                                if resolve.is_ok() {
                                    match Uri::parse(&suggestion, UriFlags::NONE) {
                                        Ok(uri) => Err(uri),
                                        Err(_) => Err(search(&profile, &query)),
                                    }
                                } else {
                                    Err(search(&profile, &query))
                                },
                            )
                        }
                    },
                ),
                Err(_) => callback(feature, cancellable, Err(search(profile, query))),
            }
        }
    }
}

/// Convert `query` to default search provider [Uri](https://docs.gtk.org/glib/struct.Uri.html)
fn search(profile: &Profile, query: &str) -> Uri {
    Uri::parse(
        &format!(
            "{}?{}",
            profile.search.default().unwrap().query, // @TODO handle
            Uri::escape_string(query, None, false)
        ),
        UriFlags::NONE,
    )
    .unwrap() // @TODO handle or skip extra URI parse by String return
}

/// Make new history record in related components
/// * optional [Uri](https://docs.gtk.org/glib/struct.Uri.html) reference wanted only for performance reasons, to not parse it twice
fn snap_history(page: &Page, uri: Option<&Uri>) {
    let request = page.navigation.request();

    // Add new record into the global memory index (used in global menu)
    // * if the `Uri` is `None`, try parse it from `request`
    match uri {
        Some(uri) => page.profile.history.memory.request.set(uri.clone()),
        None => {
            // this case especially useful for some routes that contain redirects
            // maybe some parental optimization wanted @TODO
            if let Some(uri) = page.navigation.uri() {
                page.profile.history.memory.request.set(uri);
            }
        }
    }

    // Add new record into the page navigation history
    page.item_action.history.add(request, true)
}
