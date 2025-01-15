/// Special features enumeration
/// * may not be available for some protocols
pub enum Feature {
    /// Common feature for protocol selected (e.g. browser view)
    Default { request: String },
    /// Download request with externally selected method (e.g. to file)
    Download { request: String },
    /// View request as the source (like `source-view`)
    Source { request: String },
}

impl Feature {
    // Constructors

    /// Parse new `Self` from string
    /// * holds related with parsed feature data
    pub fn from_string(request: &str) -> Self {
        if let Some(postfix) = request.strip_prefix("download:") {
            return Self::Download {
                request: postfix.to_string(),
            };
        }

        if let Some(postfix) = request.strip_prefix("source:") {
            return Self::Source {
                request: postfix.to_string(),
            };
        }

        Self::Default {
            request: request.to_string(),
        }
    }
}
