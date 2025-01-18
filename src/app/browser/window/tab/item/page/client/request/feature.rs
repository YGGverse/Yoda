// Feature conversion prefixes
const DOWNLOAD: &str = "download:";
const SOURCE: &str = "source:";

/// Feature wrapper for client `Request`
#[derive(Default)]
pub enum Feature {
    #[default]
    Default,
    Download,
    Source,
    // @TODO System(Action)
}

impl Feature {
    // Constructors

    /// Parse new `Self` from navigation entry request
    pub fn parse(request: &str) -> (Self, &str) {
        if let Some(postfix) = request.strip_prefix(DOWNLOAD) {
            return (Self::Download, postfix);
        }

        if let Some(postfix) = request.strip_prefix(SOURCE) {
            return (Self::Source, postfix);
        }

        (Self::Default, request)
    }

    // Getters

    /// Get `Self` as prefix
    pub fn as_prefix(&self) -> Option<&str> {
        match self {
            Self::Download => Some(DOWNLOAD),
            Self::Source => Some(SOURCE),
            Self::Default => None,
        }
    }
}
