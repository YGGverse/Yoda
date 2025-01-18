/// Global dependencies
use super::client::{status::Gemini, Status as Client};
use gtk::glib::DateTime;

/// `Page` status
pub enum Status {
    Client(Client),
    Failure { time: DateTime },
    Input { time: DateTime },
    Loading { time: DateTime },
    New { time: DateTime },
    Redirect { time: DateTime },
    SessionRestore { time: DateTime },
    SessionRestored { time: DateTime },
    Success { time: DateTime },
}

impl Status {
    // Getters

    /// Translate `Self` to `progress-fraction` presentation
    /// * see also: [Entry](https://docs.gtk.org/gtk4/property.Entry.progress-fraction.html)
    pub fn to_progress_fraction(&self) -> Option<f64> {
        match self {
            Self::Loading { .. } | Self::SessionRestore { .. } => Some(0.0),
            Self::Client(status) => match status {
                Client::Cancellable { .. }
                | Client::Cancelled { .. }
                | Client::Failure { .. }
                | Client::Request { .. } => Some(0.0),
                Client::Gemini(status) => match status {
                    Gemini::Resolving { .. } => Some(0.1),
                    Gemini::Resolved { .. } => Some(0.2),
                    Gemini::Connecting { .. } => Some(0.3),
                    Gemini::Connected { .. } => Some(0.4),
                    Gemini::ProxyNegotiating { .. } => Some(0.5),
                    Gemini::ProxyNegotiated { .. } => Some(0.6),
                    Gemini::TlsHandshaking { .. } => Some(0.7),
                    Gemini::TlsHandshaked { .. } => Some(0.8),
                    Gemini::Complete { .. } => Some(0.9),
                },
            },
            Self::Failure { .. }
            | Self::Success { .. }
            | Self::Redirect { .. }
            | Self::Input { .. } => Some(1.0),
            Self::New { .. } | Self::SessionRestored { .. } => None,
        }
    }
}
