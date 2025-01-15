use gtk::glib::{DateTime, GString};
use std::fmt::{Display, Formatter, Result};

/// Local `Client` status
/// * not same as the Gemini status!
pub enum Status {
    /// Ready to use (or cancel from outside)
    Cancellable(DateTime),
    /// Operation cancelled, new `Cancellable` required to continue
    Cancelled(DateTime),
    /// Redirection count limit reached by protocol driver or global settings
    RedirectLimit((DateTime, usize)),
    /// New `request` begin
    Request((DateTime, String)),
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Cancellable(t) => {
                write!(
                    f,
                    "[{}] Ready to use (or cancel from outside)",
                    format_time(t)
                )
            }
            Self::Cancelled(t) => {
                write!(
                    f,
                    "[{}] Operation cancelled, new `Cancellable` required to continue",
                    format_time(t)
                )
            }
            Self::RedirectLimit((t, count)) => {
                write!(f, "[{}] Redirection count limit ({count}) reached by protocol driver or global settings",
                format_time(t))
            }
            Self::Request((t, value)) => {
                write!(f, "[{}] Request `{value}`...", format_time(t))
            }
        }
    }
}

/// Format given [DateTime](https://docs.gtk.org/glib/struct.DateTime.html)
fn format_time(t: &DateTime) -> GString {
    t.format_iso8601().unwrap() // @TODO handle?
}
