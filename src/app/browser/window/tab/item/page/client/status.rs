mod failure;

// Children dependencies
use failure::Failure;

// Global dependencies
use gtk::glib::{DateTime, GString};
use std::fmt::{Display, Formatter, Result};

/// Local `Client` status
/// * not same as the Gemini status!
pub enum Status {
    /// Ready to use (or cancel from outside)
    Cancellable { event: DateTime },
    /// Operation cancelled, new `Cancellable` required to continue
    Cancelled { event: DateTime },
    /// Something went wrong
    Failure { event: DateTime, failure: Failure },
    /// New `request` begin
    Request { event: DateTime, value: String },
}

impl Status {
    // Constructors

    /// Create new `Self::Cancellable`
    pub fn cancellable() -> Self {
        Self::Cancellable { event: now() }
    }

    /// Create new `Self::Cancelled`
    pub fn cancelled() -> Self {
        Self::Cancelled { event: now() }
    }

    /// Create new `Self::Failure` as `Failure::RedirectLimit`
    pub fn failure_redirect_limit(count: usize) -> Self {
        Self::Failure {
            event: now(),
            failure: Failure::redirect_limit(count),
        }
    }

    /// Create new `Self::Request`
    pub fn request(value: String) -> Self {
        Self::Request {
            event: now(),
            value,
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Cancellable { event } => {
                write!(
                    f,
                    "[{}] Ready to use (or cancel from outside)",
                    format_time(event)
                )
            }
            Self::Cancelled { event } => {
                write!(
                    f,
                    "[{}] Operation cancelled, new `Cancellable` required to continue",
                    format_time(event)
                )
            }
            Self::Failure { event, failure } => {
                write!(f, "[{}] Failure: {failure}", format_time(event))
            }
            Self::Request { event, value } => {
                write!(f, "[{}] Request `{value}`...", format_time(event))
            }
        }
    }
}

/// Format given [DateTime](https://docs.gtk.org/glib/struct.DateTime.html)
fn format_time(t: &DateTime) -> GString {
    t.format_iso8601().unwrap() // @TODO handle?
}

/// Get current [DateTime](https://docs.gtk.org/glib/struct.DateTime.html)
fn now() -> DateTime {
    DateTime::now_local().unwrap() // @TODO handle?
}
