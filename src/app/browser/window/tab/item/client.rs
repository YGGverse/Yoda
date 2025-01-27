mod driver;
mod feature;
mod subject;

use super::Page;
use adw::TabPage;
use driver::Driver;
use feature::Feature;
use gtk::{
    gio::Cancellable,
    glib::{Uri, UriFlags},
    prelude::{ActionExt, CancellableExt, EntryExt},
};
use std::{cell::Cell, rc::Rc};
use subject::Subject;

/// Multi-protocol client API for tab `Item`
pub struct Client {
    cancellable: Cell<Cancellable>,
    driver: Rc<Driver>,
    subject: Rc<Subject>,
}

impl Client {
    // Constructors

    /// Create new `Self`
    pub fn init(page: &Rc<Page>, tab_page: &TabPage) -> Self {
        let subject = Rc::new(Subject {
            page: page.clone(),
            tab_page: tab_page.clone(),
        });

        Self {
            cancellable: Cell::new(Cancellable::new()),
            driver: Rc::new(Driver::build(&subject)),
            subject,
        }
    }

    // Actions

    /// Route tab item `request` to protocol driver
    /// * or `navigation` entry if the value not provided
    pub fn handle(&self, request: &str, is_snap_history: bool) {
        // Move focus out from navigation entry @TODO
        self.subject.page.browser_action.escape.activate(None);

        // Initially disable find action
        self.subject
            .page
            .window_action
            .find
            .simple_action
            .set_enabled(false);

        // Reset widgets
        self.subject.page.search.unset();
        self.subject.page.input.unset();
        self.subject.tab_page.set_title("Loading..");
        self.subject
            .page
            .navigation
            .request
            .set_progress_fraction(0.1);

        self.subject.tab_page.set_loading(true);

        if is_snap_history {
            snap_history(&self.subject, None);
        }

        // run async resolver to detect Uri, scheme-less host, or search query
        lookup(request, self.cancellable(), {
            let driver = self.driver.clone();
            let subject = self.subject.clone();
            move |feature, cancellable, result| {
                match result {
                    // route by scheme
                    Ok(uri) => match uri.scheme().as_str() {
                        "gemini" | "titan" => driver.gemini.handle(uri, feature, cancellable),
                        scheme => {
                            // no scheme match driver, complete with failure message
                            let status = subject.page.content.to_status_failure();
                            status.set_description(Some(&format!(
                                "Scheme `{scheme}` yet not supported"
                            )));
                            subject.tab_page.set_title(&status.title());
                            subject.page.navigation.request.set_progress_fraction(0.0);
                            subject.tab_page.set_loading(false);
                        }
                    },
                    // begin redirection to new address suggested
                    Err(uri) => subject
                        .page
                        .item_action
                        .load
                        .activate(Some(&uri.to_string()), false),
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
    const TIMEOUT: u32 = 250; // ms

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
                        let query = query.to_owned();
                        move |resolve| {
                            callback(
                                feature,
                                cancellable,
                                if resolve.is_ok() {
                                    match Uri::parse(&suggestion, UriFlags::NONE) {
                                        Ok(uri) => Err(uri),
                                        Err(_) => Err(search(&query)),
                                    }
                                } else {
                                    Err(search(&query))
                                },
                            )
                        }
                    },
                ),
                Err(_) => callback(feature, cancellable, Err(search(query))),
            }
        }
    }
}

/// Convert `query` to default search provider [Uri](https://docs.gtk.org/glib/struct.Uri.html)
fn search(query: &str) -> Uri {
    Uri::build(
        UriFlags::NONE,
        "gemini",
        None,
        Some("kennedy.gemi.dev"), // tlgs.one was replaced by response time issue
        -1,
        "/search",
        Some(&Uri::escape_string(query, None, false)),
        None,
    ) // @TODO optional settings
}

/// Make new history record in related components
/// * optional [Uri](https://docs.gtk.org/glib/struct.Uri.html) reference wanted only for performance reasons, to not parse it twice
fn snap_history(subject: &Rc<Subject>, uri: Option<&Uri>) {
    let request = subject.page.navigation.request();

    // Add new record into the global memory index (used in global menu)
    // * if the `Uri` is `None`, try parse it from `request`
    match uri {
        Some(uri) => subject.page.profile.history.memory.request.set(uri.clone()),
        None => {
            // this case especially useful for some routes that contain redirects
            // maybe some parental optimization wanted @TODO
            if let Some(uri) = subject.page.navigation.uri() {
                subject.page.profile.history.memory.request.set(uri);
            }
        }
    }

    // Add new record into the page navigation history
    subject.page.item_action.history.add(request, true)
}
