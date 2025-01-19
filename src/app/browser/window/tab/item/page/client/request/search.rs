//! Search providers asset

use gtk::glib::{Uri, UriFlags};

/// Default search provider
pub fn tgls(query: &str) -> Uri {
    Uri::build(
        UriFlags::NONE,
        "gemini",
        None,
        Some("tlgs.one"),
        -1,
        "/search",
        Some(&Uri::escape_string(query, None, false)),
        None,
    )
}
