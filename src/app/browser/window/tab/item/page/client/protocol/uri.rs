//! Search providers [Uri](https://docs.gtk.org/glib/struct.Uri.html) asset

// Global dependencies
use gtk::glib::{Uri, UriFlags};

/// Build TGLS [Uri](https://docs.gtk.org/glib/struct.Uri.html)
pub fn tgls(request: &str) -> Uri {
    Uri::build(
        UriFlags::NONE,
        "gemini",
        None,
        Some("tlgs.one"),
        1965,
        "search",
        Some(&Uri::escape_string(request, None, false)), // @TODO is `escape_string` really wanted in `build` context?
        None,
    )
}
