mod redirect;
mod status;

use redirect::Redirect;
use status::Status;

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
            status: Rc::new(RefCell::new(Status::Cancellable)),
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
            self.status.replace(Status::Cancelled);
        } else {
            self.status.replace(Status::Cancellable);
        }

        // Done
        cancellable
    }

    pub fn request(&self, query: &str) {
        self.status.replace(Status::Request(query.to_string()));

        // Forcefully prevent infinitive redirection
        // * this condition just to make sure that client will never stuck by driver implementation issue
        if self.redirect.count() > redirect::LIMIT {
            self.status.replace(Status::RedirectLimit(redirect::LIMIT));
            // @TODO return;
        }
    }
}
