use gtk::glib::Uri;

/// Failure type for client `Response`
pub enum Failure {
    Status {
        message: String,
    },
    /// This failure type provides `base` member to build Download page
    /// for the constructed request [Uri](https://docs.gtk.org/glib/struct.Uri.html)
    Mime {
        base: Uri,
        mime: String,
        message: String,
    },
    /// Common error type
    Error {
        message: String,
    },
}
