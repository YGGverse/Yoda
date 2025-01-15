//! Feature components in development,
//! this asset initiated as the attempt to reduce current `Page` code size
//! and delegate different protocol features to specified drivers under this location with itself implementation
//  @TODO cleanup this message on complete

mod request;

// Local dependencies
use request::Request;

/// Features route for `Client`
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
    /// * holds related with parsed feature data
    pub fn from_string(request: &str) -> Self {
        if let Some(postfix) = request.strip_prefix("download:") {
            return Self::Download {
                request: Request::from_string(postfix),
            };
        }

        if let Some(postfix) = request.strip_prefix("source:") {
            return Self::Source {
                request: Request::from_string(postfix),
            };
        }

        Self::Default {
            request: Request::from_string(request),
        }
    }
}
