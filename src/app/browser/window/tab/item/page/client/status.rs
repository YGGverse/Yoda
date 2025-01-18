pub mod failure;
pub mod gemini;

// Children dependencies
pub use failure::Failure;
pub use gemini::Gemini;

// Global dependencies
use crate::tool::format_time;
use gtk::glib::DateTime;
use std::fmt::{Display, Formatter, Result};

/// Local `Client` status
/// * not same as the Gemini status!
pub enum Status {
    /// Ready to use (or cancel from outside)
    Cancellable { time: DateTime },
    /// Operation cancelled, new `Cancellable` required to continue
    Cancelled { time: DateTime },
    /// Protocol driver updates
    Gemini(Gemini),
    /// Something went wrong
    Failure { time: DateTime, failure: Failure },
    /// New `request` begin
    Request { time: DateTime, value: String },
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Cancellable { time } => {
                write!(
                    f,
                    "[{}] Ready to use (or cancel from outside)",
                    format_time(time)
                )
            }
            Self::Cancelled { time } => {
                write!(
                    f,
                    "[{}] Operation cancelled, new `Cancellable` required to continue",
                    format_time(time)
                )
            }
            Self::Gemini(status) => {
                write!(f, "{status}")
            }
            Self::Failure { time, failure } => {
                write!(f, "[{}] Failure: {failure}", format_time(time))
            }
            Self::Request { time, value } => {
                write!(f, "[{}] Request `{value}`...", format_time(time))
            }
        }
    }
}
