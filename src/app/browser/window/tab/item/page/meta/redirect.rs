use gtk::glib::GString;

/// # Redirection data holder
///
/// This component does nothing,
/// but useful as the container for temporary redirection data
/// operated by external controller
///
/// ## Members
///
/// * `is_foreground` - indicates how to process this redirect
/// * `request` - destination
///   * currently, it's raw `GString` not [Uri](https://docs.gtk.org/glib/struct.Uri.html)
///     because of compatibility with request field as it could contain any other, not parsable values
#[derive(Clone, Debug)]
pub struct Redirect {
    pub is_foreground: bool,
    pub referrer: Option<GString>,
    pub request: GString,
}

impl Redirect {
    // Constructors

    pub fn new(request: GString, referrer: Option<GString>, is_foreground: bool) -> Self {
        Self {
            is_foreground,
            referrer,
            request,
        }
    }
}
