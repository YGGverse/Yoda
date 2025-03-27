pub mod method;
pub use method::Method;

use super::Info;

/// Unified redirection info wrapper for the application page
pub struct Redirect {
    pub referrer: Box<Info>,
    pub method: Method,
}

impl Redirect {
    // Constructors

    pub fn permanent(referrer: Info) -> Self {
        Self {
            referrer: Box::new(referrer),
            method: Method::Permanent,
        }
    }

    pub fn temporary(referrer: Info) -> Self {
        Self {
            referrer: Box::new(referrer),
            method: Method::Temporary,
        }
    }

    // Getters

    /// Check redirection has external target
    /// * return `None` when at least one request value could not be parsed to
    ///   the valid [Uri](https://docs.gtk.org/glib/struct.Uri.html) host
    pub fn is_external(&self, cmp: &Info) -> Option<bool> {
        fn parse(info: &Info) -> Option<gtk::glib::GString> {
            gtk::glib::Uri::parse(info.request.as_ref()?, gtk::glib::UriFlags::NONE)
                .ok()?
                .host()
        }
        Some(parse(&self.referrer)? != parse(cmp)?)
    }
}
