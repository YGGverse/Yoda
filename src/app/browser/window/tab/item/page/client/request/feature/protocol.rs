// Global dependencies
use gtk::{
    gio::Cancellable,
    glib::{Priority, Uri, UriFlags},
};

#[derive(Clone)]
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
    Undefined,
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
                _ => Self::Undefined,
            },
            // Search request if the request could not be parsed as the valid [URI](https://docs.gtk.org/glib/struct.Uri.html)
            // * @TODO implement DNS lookup before apply this option
            Err(_) => Self::Gemini {
                uri: Uri::build(
                    UriFlags::NONE,
                    "gemini",
                    None,
                    Some("tlgs.one"),
                    -1,
                    "/search", // beginning slash required to prevent assertion panic on construct
                    Some(&Uri::escape_string(query, None, false)), // @TODO is `escape_string` really wanted in `build` context?
                    None,
                ),
                cancellable,
                priority,
            },
        }
    }

    // Getters

    pub fn uri(&self) -> Option<&Uri> {
        match self {
            Self::Gemini {
                uri,
                cancellable: _,
                priority: _,
            }
            | Self::Titan {
                uri,
                cancellable: _,
                priority: _,
            } => Some(&uri),
            Self::Undefined => None,
        }
    }
}
