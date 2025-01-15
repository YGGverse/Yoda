use std::fmt::{Display, Formatter, Result};

/// Local `Client` status
/// * not same as the Gemini status!
pub enum Status {
    /// Ready to use (or cancel from outside)
    Cancellable,
    /// Operation cancelled, new `Cancellable` required to continue
    Cancelled,
    /// Redirection count limit reached by protocol driver or global settings
    RedirectLimit(usize),
    /// New `request` begin
    Request(String),
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Cancellable => {
                write!(f, "Ready to use (or cancel from outside)")
            }
            Self::Cancelled => {
                write!(
                    f,
                    "Operation cancelled, new `Cancellable` required to continue"
                )
            }
            Self::RedirectLimit(count) => {
                write!(f, "Redirection count limit ({count}) reached by protocol driver or global settings")
            }
            Self::Request(value) => {
                write!(f, "Request `{value}`...")
            }
        }
    }
}
