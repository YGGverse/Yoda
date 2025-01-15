mod feature;
mod uri;

use feature::Feature;

use gtk::glib::{Uri, UriFlags};

pub enum Protocol {
    Gemini { /*mode: Mode,*/ uri: Uri },
    Titan { /*mode: Mode,*/ uri: Uri },
    Undefined,
}

impl Protocol {
    // Constructors

    /// Create new `Self` from parsable request string
    pub fn from_string(request: &str) -> Self {
        match Feature::from_string(request) {
            Feature::Default { request }
            | Feature::Download { request }
            | Feature::Source { request } => match Uri::parse(&request, UriFlags::NONE) {
                Ok(uri) => match uri.scheme().as_str() {
                    "gemini" => Self::Gemini { uri },
                    "titan" => Self::Titan { uri },
                    _ => Self::Undefined,
                },
                Err(_) => Self::Gemini {
                    uri: uri::tgls(&request),
                },
            },
        }
    }
}
