use gtk::glib::{Uri, UriFlags};

/// Request type for `Page` with optional value parsed
pub enum Request {
    Default(Uri),
    Download(Uri),
    Source(Uri),
    Search(String),
}

impl Request {
    // Constructors

    /// Create new `Self` from `request` string
    pub fn from(request: &str) -> Self {
        if let Some(postfix) = request.strip_prefix("source:") {
            if let Ok(uri) = Uri::parse(postfix, UriFlags::NONE) {
                return Self::Source(uri);
            }
        }

        if let Some(postfix) = request.strip_prefix("download:") {
            if let Ok(uri) = Uri::parse(postfix, UriFlags::NONE) {
                return Self::Download(uri);
            }
        }

        if let Ok(uri) = Uri::parse(request, UriFlags::NONE) {
            return Self::Default(uri);
        }

        Self::Search(request.to_string())
    }
}
