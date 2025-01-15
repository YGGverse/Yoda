pub enum Feature {
    Default { request: String },
    Download { request: String },
    Source { request: String },
}

impl Feature {
    // Constructors

    /// Parse new `Self` from string
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
