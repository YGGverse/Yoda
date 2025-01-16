/// Global dependencies
use super::client::{driver::Status as Driver, Status as Client};
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
                Client::Driver(status) => match status {
                    Driver::Resolving { .. } => Some(0.1),
                    Driver::Resolved { .. } => Some(0.2),
                    Driver::Connecting { .. } => Some(0.3),
                    Driver::Connected { .. } => Some(0.4),
                    Driver::ProxyNegotiating { .. } => Some(0.5),
                    Driver::ProxyNegotiated { .. } => Some(0.6),
                    Driver::TlsHandshaking { .. } => Some(0.7),
                    Driver::TlsHandshaked { .. } => Some(0.8),
                    Driver::Complete { .. } => Some(0.9),
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
