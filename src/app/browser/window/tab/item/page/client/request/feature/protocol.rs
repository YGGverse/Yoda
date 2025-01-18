// Global dependencies
use gtk::{
    gio::Cancellable,
    glib::{Priority, Uri, UriFlags},
};

pub enum Protocol {
    Gemini {
        uri: Uri,
        cancellable: Cancellable,
        priority: Priority,
    },
    Titan {
        uri: Uri,
        cancellable: Cancellable,
        priority: Priority,
    },
    Unsupported,
}

impl Protocol {
    // Constructors

    /// Create new `Self` from parsable request string
    pub fn build(query: &str, cancellable: Cancellable, priority: Priority) -> Self {
        match Uri::parse(query, UriFlags::NONE) {
            Ok(uri) => match uri.scheme().as_str() {
                "gemini" => Self::Gemini {
                    uri,
                    cancellable,
                    priority,
                },
                "titan" => Self::Titan {
                    uri,
                    cancellable,
                    priority,
                },
                _ => Self::Unsupported,
            },
            // Search request if the request could not be parsed as the valid [URI](https://docs.gtk.org/glib/struct.Uri.html)
            // * @TODO implement DNS lookup before apply this option
            Err(_) => Self::Gemini {
                uri: Uri::build(
                    UriFlags::NONE,
                    "gemini",
                    None,
                    Some("tlgs.one"),
                    1965,
                    "search",
                    Some(&Uri::escape_string(query, None, false)), // @TODO is `escape_string` really wanted in `build` context?
                    None,
                ),
                cancellable,
                priority,
            },
        }
    }
}
