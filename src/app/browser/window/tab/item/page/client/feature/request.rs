mod search;

// Global dependencies
use gtk::glib::{Uri, UriFlags};

pub enum Request {
    Gemini { uri: Uri },
    Titan { uri: Uri },
    Undefined,
}

impl Request {
    // Constructors

    /// Create new `Self` from parsable request string
    pub fn from_string(request: &str) -> Self {
        match Uri::parse(request, UriFlags::NONE) {
            Ok(uri) => match uri.scheme().as_str() {
                "gemini" => Self::Gemini { uri },
                "titan" => Self::Titan { uri },
                _ => Self::Undefined,
            },
            // Search request if the request could not be parsed as the valid [URI](https://docs.gtk.org/glib/struct.Uri.html)
            // * @TODO implement DNS resolver lookup before assign this option
            Err(_) => Self::Gemini {
                uri: search::tgls(request),
            },
        }
    }

    // Actions

    /// Send request using protocol driver constructed
    pub fn send(&self) {
        match self {
            Request::Gemini { uri } => todo!("{uri}"),
            Request::Titan { uri } => todo!("{uri}"),
            Request::Undefined => todo!(),
        }
    }
}
