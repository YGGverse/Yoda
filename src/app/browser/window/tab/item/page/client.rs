pub mod driver;
pub mod request;
pub mod response;
pub mod status;

// Children dependencies
pub use driver::Driver;
pub use request::Request;
pub use response::Response;
pub use status::Status;

// Global dependencies
use crate::{tool::now, Profile};
use gtk::{gio::Cancellable, glib::Priority, prelude::CancellableExt};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

/// Multi-protocol client API for `Page` object
pub struct Client {
    cancellable: Cell<Cancellable>,
    status: Rc<RefCell<Status>>,
    driver: Driver,
}

impl Client {
    // Constructors

    /// Create new `Self`
    pub fn init(profile: &Rc<Profile>, callback: impl Fn(Status) + 'static) -> Self {
        Self {
            cancellable: Cell::new(Cancellable::new()),
            driver: Driver::init(profile.clone(), move |status| {
                callback(Status::Driver(status))
            }),
            status: Rc::new(RefCell::new(Status::Cancellable { time: now() })), // e.g. "ready to use"
        }
    }

    // Actions

    /// Begin new request
    /// * the `query` as string, to support system routes (e.g. `source:` prefix)
    pub fn request_async(&self, query: &str, callback: impl FnOnce(Response) + 'static) {
        // Update client status
        self.status.replace(Status::Request {
            time: now(),
            value: query.to_string(),
        });

        self.driver.request_async(
            Request::build(query, None, self.new_cancellable(), Priority::DEFAULT),
            callback,
        );
    }

    /// Get new [Cancellable](https://docs.gtk.org/gio/class.Cancellable.html) by cancel previous one
    fn new_cancellable(&self) -> Cancellable {
        // Init new Cancellable
        let cancellable = Cancellable::new();

        // Replace by cancel previous operations
        let previous = self.cancellable.replace(cancellable.clone());
        if !previous.is_cancelled() {
            previous.cancel();
            self.status.replace(Status::Cancelled { time: now() });
        } else {
            self.status.replace(Status::Cancellable { time: now() });
        }

        // Done
        cancellable
    }
}
