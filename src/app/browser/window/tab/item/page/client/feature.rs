pub mod request;
pub use request::Request;

/// Feature wrapper for client `Request`
pub enum Feature {
    /// Common feature for protocol selected (e.g. browser view)
    Default { request: Request },
    /// Download request with externally selected method (e.g. to file)
    Download { request: Request },
    /// View request as the source (like `source-view`)
    Source { request: Request },
}

impl Feature {
    // Constructors

    /// Parse new `Self` from string
    pub fn from_string(query: &str) -> Self {
        if let Some(postfix) = query.strip_prefix("download:") {
            return Self::Download {
                request: Request::from_string(postfix),
            };
        }

        if let Some(postfix) = query.strip_prefix("source:") {
            return Self::Source {
                request: Request::from_string(postfix),
            };
        }

        Self::Default {
            request: Request::from_string(query),
        }
    }
}
