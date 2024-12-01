use gtk::gio::Cancellable;
use std::cell::RefCell;

/// Multi-client holder for single `Page` object
///
/// Unlike init new client instance on every page load,
/// this struct creates single holder for different protocol drivers;
/// it also provides additional client-side features
/// e.g. session resumption or multi-thread connection management (depending of client type selected)
pub struct Client {
    // Shared reference to cancel async operations
    pub cancellable: RefCell<Cancellable>,
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
}
