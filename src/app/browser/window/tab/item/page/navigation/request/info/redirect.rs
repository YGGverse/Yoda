pub mod method;
pub use method::Method;

use super::Info;

/// Unified redirection info wrapper for the application page
pub struct Redirect {
    pub info: Info,
    pub method: Method,
}

impl Redirect {
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
        Some(parse(&self.info)? != parse(cmp)?)
    }
}
