mod driver;
mod feature;

use super::{Page, Profile};
use driver::Driver;
use feature::Feature;
use gtk::{
    gio::Cancellable,
    glib::{Uri, UriFlags},
    prelude::CancellableExt,
};
use std::{cell::Cell, rc::Rc, sync::Arc};

/// Multi-protocol client API for tab `Item`
pub struct Client {
    cancellable: Cell<Cancellable>,
    driver: Rc<Driver>,
    page: Rc<Page>,
    profile: Arc<Profile>,
}

impl Client {
    // Constructors

    /// Create new `Self`
    pub fn init(profile: &Arc<Profile>, page: &Rc<Page>) -> Self {
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
    pub fn handle(&self, request: &str, is_snap_history: bool, is_redirect: bool) {
        self.page.escape();

        // Initially disable find action
        self.page
            .window_action
            .find
            .simple_action
            .set_enabled(false);

        // Reset widgets
        self.page.input.unset();
        self.page.search.unset();
        self.page.set_title("Loading..");
        self.page.set_progress(0.1);
        self.page
            .navigation
            .request
            .info
            .borrow_mut()
            .reset(!is_redirect);

        // run async resolver to detect Uri, scheme-less host, or search query
        lookup(&self.profile, request, self.cancellable(), {
            let driver = self.driver.clone();
            let page = self.page.clone();
            move |feature, cancellable, result| {
                match result {
                    // route by scheme
                    Ok(uri) => match uri.scheme().as_str() {
                        "file" => driver
                            .file
                            .handle(uri, feature, cancellable, is_snap_history),
                        "gemini" | "titan" => {
                            driver
                                .gemini
                                .handle(uri, feature, cancellable, is_snap_history)
                        }
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
                    Err(query) => {
                        page.item_action
                            .load
                            .activate(Some(&query), is_snap_history, true)
                    }
                }
            }
        })
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
    profile: &Arc<Profile>,
    query: &str,
    cancellable: Cancellable,
    callback: impl FnOnce(Rc<Feature>, Cancellable, Result<Uri, String>) + 'static,
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
                                    Err(match Uri::parse(&suggestion, UriFlags::NONE) {
                                        Ok(uri) => uri,
                                        Err(_) => search(&profile, &query),
                                    }
                                    .to_string())
                                } else {
                                    const FILE_SCHEME: &str = "file://";
                                    Err(
                                        // try autocomplete scheme if the request match local filename
                                        if !query.starts_with(FILE_SCHEME)
                                            && std::path::Path::new(&query).exists()
                                        {
                                            format!("{FILE_SCHEME}{query}")
                                        } else {
                                            search(&profile, &query).to_string()
                                        },
                                    )
                                },
                            )
                        }
                    },
                ),
                Err(_) => callback(
                    feature,
                    cancellable,
                    Err(search(profile, query).to_string()),
                ),
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
