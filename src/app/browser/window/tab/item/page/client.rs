use gtk::{gio::Cancellable, prelude::CancellableExt};
use std::cell::RefCell;

/// Multi-client holder for single `Page` object
///
/// Unlike init new client instance on every page load,
/// this struct creates single holder for different protocol drivers;
/// it also provides additional client-side features
/// e.g. session resumption or multi-thread connection management (depending of client type selected)
pub struct Client {
    // Shared reference to cancel async operations
    cancellable: RefCell<Cancellable>,
    // Clients
    pub gemini: gemini::Client,
    // other clients..
}

impl Client {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            cancellable: RefCell::new(Cancellable::new()),
            gemini: gemini::Client::new(),
        }
    }

    /// Get new [Cancellable](https://docs.gtk.org/gio/class.Cancellable.html) by cancel previous one
    pub fn cancellable(&self) -> Cancellable {
        // Init new Cancellable
        let cancellable = Cancellable::new();

        // Replace by cancel previous operations
        let previous = self.cancellable.replace(cancellable.clone());
        if !previous.is_cancelled() {
            previous.cancel();
        }

        // Done
        cancellable
    }
}
