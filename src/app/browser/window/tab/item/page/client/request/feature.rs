pub mod protocol;
pub use protocol::Protocol;

use gtk::{gio::Cancellable, glib::Priority};

/// Feature wrapper for client `Request`
pub enum Feature {
    Default(Protocol),
    Download(Protocol),
    Source(Protocol),
    // @TODO System(Action)
}

impl Feature {
    // Constructors

    /// Parse new `Self` from string
    pub fn build(query: &str, cancellable: Cancellable, priority: Priority) -> Self {
        if let Some(postfix) = query.strip_prefix("download:") {
            return Self::Download(Protocol::build(postfix, cancellable, priority));
        }

        if let Some(postfix) = query.strip_prefix("source:") {
            return Self::Source(Protocol::build(postfix, cancellable, priority));
        }

        Self::Default(Protocol::build(query, cancellable, priority))
    }
}
