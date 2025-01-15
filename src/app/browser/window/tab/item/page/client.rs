mod redirect;
mod status;

// Children dependencies
use redirect::Redirect;
use status::Status;

// Global dependencies
use gtk::{gio::Cancellable, prelude::CancellableExt};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

/// Multi-client holder for single `Page` object
///
/// Unlike init new client instance on every page load,
/// this struct creates single holder for different protocol drivers;
/// it also provides additional client-side features
/// e.g. session resumption or multi-thread connection management (depending of client type selected)
pub struct Client {
    // Shared reference to cancel async operations
    // * keep it private to make sure that `status` member tracked properly
    cancellable: Cell<Cancellable>,
    // Redirects resolver for different protocols
    pub redirect: Rc<Redirect>,
    // Track update status
    status: Rc<RefCell<Status>>,
    // Drivers
    pub gemini: gemini::Client,
    // other clients..
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            cancellable: Cell::new(Cancellable::new()),
            redirect: Rc::new(Redirect::new()),
            status: Rc::new(RefCell::new(Status::cancellable())), // e.g. "ready to use"
            gemini: gemini::Client::new(),
        }
    }

    // Actions

    /// Get new [Cancellable](https://docs.gtk.org/gio/class.Cancellable.html) by cancel previous one
    /// * this action wanted just because of `Cancelable` member constructed privately,
    ///   where some external components may depend to sync their related processes
    pub fn cancellable(&self) -> Cancellable {
        // Init new Cancellable
        let cancellable = Cancellable::new();

        // Replace by cancel previous operations
        let previous = self.cancellable.replace(cancellable.clone());
        if !previous.is_cancelled() {
            previous.cancel();
            self.status.replace(Status::cancelled());
        } else {
            self.status.replace(Status::cancellable());
        }

        // Done
        cancellable
    }

    /// Begin new request
    /// * the `query` as string, to support system routing requests (e.g. `source:`)
    pub fn request(&self, query: &str) {
        self.status.replace(Status::request(query.to_string()));

        // Forcefully prevent infinitive redirection
        // * this condition just to make sure that client will never stuck by driver implementation issue
        if self.redirect.count() > redirect::LIMIT {
            self.status
                .replace(Status::failure_redirect_limit(redirect::LIMIT));
            // @TODO return;
        }
    }
}
