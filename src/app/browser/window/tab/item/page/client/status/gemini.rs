// Global dependencies
use crate::tool::format_time;
use gtk::glib::DateTime;
use std::fmt::{Display, Formatter, Result};

/// Shared asset for `Gemini` statuses
pub enum Gemini {
    Resolving { time: DateTime },
    Resolved { time: DateTime },
    Connecting { time: DateTime },
    Connected { time: DateTime },
    ProxyNegotiating { time: DateTime },
    ProxyNegotiated { time: DateTime },
    TlsHandshaking { time: DateTime },
    TlsHandshaked { time: DateTime },
    Complete { time: DateTime },
}

impl Display for Gemini {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Resolving { time } => {
                write!(f, "[{}] Resolving", format_time(time))
            }
            Self::Resolved { time } => {
                write!(f, "[{}] Resolved", format_time(time))
            }
            Self::Connecting { time } => {
                write!(f, "[{}] Connecting", format_time(time))
            }
            Self::Connected { time } => {
                write!(f, "[{}] Connected", format_time(time))
            }
            Self::ProxyNegotiating { time } => {
                write!(f, "[{}] Proxy negotiating", format_time(time))
            }
            Self::ProxyNegotiated { time } => {
                write!(f, "[{}] Proxy negotiated", format_time(time))
            }
            Self::TlsHandshaking { time } => {
                write!(f, "[{}] TLS handshaking", format_time(time))
            }
            Self::TlsHandshaked { time } => {
                write!(f, "[{}] TLS handshaked", format_time(time))
            }
            Self::Complete { time } => {
                write!(f, "[{}] Completed", format_time(time))
            }
        }
    }
}
