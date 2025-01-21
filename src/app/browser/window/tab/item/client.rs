mod driver;
mod feature;

use super::Page;
use adw::TabPage;
use driver::Driver;
use feature::Feature;
use gtk::{
    gio::Cancellable,
    glib::{Uri, UriFlags},
    prelude::CancellableExt,
};
use std::{cell::Cell, rc::Rc};

/// Multi-protocol client API for tab `Item`
pub struct Client {
    cancellable: Cell<Cancellable>,
    driver: Rc<Driver>,
}

impl Client {
    // Constructors

    /// Create new `Self`
    pub fn init(page: &Rc<Page>, tab_page: &TabPage) -> Self {
        Self {
            cancellable: Cell::new(Cancellable::new()),
            driver: Rc::new(Driver::build(page, tab_page)),
        }
    }

    // Actions

    /// Route tab item `request` to protocol driver
    /// * or `navigation` entry if the value not provided
    pub fn handle(&self, request: &str, is_snap_history: bool) {
        // run async resolver to detect Uri, scheme-less host, or search query
        lookup(
            request,
            self.driver.clone(),
            self.cancellable(),
            move |driver, feature, cancellable, uri| {
                route(driver, feature, cancellable, uri, is_snap_history)
            },
        )
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
/// * useful for scheme-less requests, before apply search redirect
/// * the `query` should not contain `feature` prefix
fn lookup(
    query: &str,
    driver: Rc<Driver>,
    cancellable: Cancellable,
    callback: impl FnOnce(Rc<Driver>, Feature, Cancellable, Uri) + 'static,
) {
    use gtk::{
        gio::{NetworkAddress, Resolver},
        prelude::{NetworkAddressExt, ResolverExt},
    };

    const DEFAULT_SCHEME: &str = "gemini";
    const DEFAULT_PORT: u16 = 1965;
    const TIMEOUT: u32 = 250; // ms

    let (feature, query) = Feature::parse(query.trim());

    match Uri::parse(query, UriFlags::NONE) {
        Ok(uri) => callback(driver, feature, cancellable, uri),
        Err(_) => {
            // try default scheme suggestion
            let suggestion = format!("{DEFAULT_SCHEME}://{query}");

            let resolver = Resolver::default();
            resolver.set_timeout(TIMEOUT);

            match NetworkAddress::parse_uri(&suggestion, DEFAULT_PORT) {
                Ok(connectable) => resolver.lookup_by_name_async(
                    &connectable.hostname(),
                    Some(&cancellable.clone()),
                    move |resolve| {
                        callback(
                            driver,
                            feature,
                            cancellable,
                            if resolve.is_ok() {
                                match Uri::parse(&suggestion, UriFlags::NONE) {
                                    Ok(uri) => uri,
                                    Err(_) => search(&suggestion),
                                }
                            } else {
                                search(&suggestion)
                            },
                        )
                    },
                ),
                Err(_) => callback(driver, feature, cancellable, search(&suggestion)),
            }
        }
    }
}

/// Route request (resolved by `lookup` function)
fn route(
    driver: Rc<Driver>,
    feature: Feature,
    cancellable: Cancellable,
    uri: Uri,
    is_snap_history: bool,
) {
    match uri.scheme().as_str() {
        "gemini" => driver
            .gemini
            .handle(uri, feature, cancellable, is_snap_history),
        _ => todo!(),
    }
}

/// Convert `query` to default search provider [Uri](https://docs.gtk.org/glib/struct.Uri.html)
fn search(query: &str) -> Uri {
    Uri::build(
        UriFlags::NONE,
        "gemini",
        None,
        Some("tlgs.one"),
        -1,
        "/search",
        Some(&Uri::escape_string(query, None, false)),
        None,
    ) // @TODO optional settings
}
