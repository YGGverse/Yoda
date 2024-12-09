use gtk::glib::{GString, Uri, UriFlags};

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
    /// * if some `referrer` given, make additional check in previous request
    pub fn from(request: &str, referrer: Option<&GString>) -> Self {
        // check in request
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

        // check in referrer @TODO tmp
        if referrer.is_some_and(|this| this.starts_with("source:")) {
            if let Ok(uri) = Uri::parse(request, UriFlags::NONE) {
                return Self::Source(uri);
            }
        }

        if referrer.is_some_and(|this| this.starts_with("download:")) {
            if let Ok(uri) = Uri::parse(request, UriFlags::NONE) {
                return Self::Download(uri);
            }
        }

        // is default
        if let Ok(uri) = Uri::parse(request, UriFlags::NONE) {
            return Self::Default(uri);
        }

        // is search
        Self::Search(request.to_string())
    }
}
