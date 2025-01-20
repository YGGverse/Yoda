pub mod request;
pub mod response;
pub mod status;

// Children dependencies
pub use request::Request;
pub use response::Response;
pub use status::Status;

// Global dependencies
use crate::{tool::now, Profile};
use gtk::{
    gio::{Cancellable, SocketClientEvent},
    prelude::{CancellableExt, SocketClientExt},
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

/// Multi-protocol client API for `Page` object
pub struct Client {
    cancellable: Cell<Cancellable>,
    status: Rc<RefCell<Status>>,
    /// Profile reference required for Gemini protocol auth (match scope)
    profile: Rc<Profile>,
    /// Supported clients
    /// * gemini driver should be initiated once (on page object init)
    ///   to process all it connection features properly
    gemini: Rc<ggemini::Client>,
}

impl Client {
    // Constructors

    /// Create new `Self`
    pub fn init(profile: &Rc<Profile>, callback: impl Fn(Status) + 'static) -> Self {
        use status::Gemini;
        // Init supported protocol libraries
        let gemini = Rc::new(ggemini::Client::new());

        // Retransmit gemini [SocketClient](https://docs.gtk.org/gio/class.SocketClient.html) updates
        gemini.socket.connect_event(move |_, event, _, _| {
            callback(Status::Gemini(match event {
                SocketClientEvent::Resolving => Gemini::Resolving { time: now() },
                SocketClientEvent::Resolved => Gemini::Resolved { time: now() },
                SocketClientEvent::Connecting => Gemini::Connecting { time: now() },
                SocketClientEvent::Connected => Gemini::Connected { time: now() },
                SocketClientEvent::ProxyNegotiating => Gemini::ProxyNegotiating { time: now() },
                SocketClientEvent::ProxyNegotiated => Gemini::ProxyNegotiated { time: now() },
                // * `TlsHandshaking` | `TlsHandshaked` has effect only for guest connections!
                SocketClientEvent::TlsHandshaking => Gemini::TlsHandshaking { time: now() },
                SocketClientEvent::TlsHandshaked => Gemini::TlsHandshaked { time: now() },
                SocketClientEvent::Complete => Gemini::Complete { time: now() },
                _ => todo!(), // alert on API change
            }))
        });

        Self {
            cancellable: Cell::new(Cancellable::new()),
            status: Rc::new(RefCell::new(Status::Cancellable { time: now() })), // e.g. "ready to use"
            profile: profile.clone(),
            gemini,
        }
    }

    // Actions

    /// Begin new request
    /// * the `query` as string, to support system routes (e.g. `source:` prefix)
    pub fn request(&self, query: &str, callback: impl FnOnce(Response) + 'static) {
        self.status.replace(Status::Request {
            time: now(),
            value: query.to_string(),
        });

        use request::Error;
        use response::{Failure, Redirect};

        let cancellable = self.new_cancellable();

        match Request::parse(query) {
            Ok(request) => request.handle(self, cancellable, callback),
            Err(e) => match e {
                // return failure response on unsupported scheme detected
                Error::Unsupported => callback(Response::Failure(Failure::Error {
                    message: "Request scheme yet not supported".to_string(),
                })),
                // try async resolver (slow method)
                _ => Request::lookup(query, Some(&cancellable), |result| {
                    callback(match result {
                        // redirection with scheme auto-complete or default search provider
                        Ok(request) => match request {
                            Request::Gemini(this, _) => {
                                Response::Redirect(Redirect::Foreground(this.uri))
                            }
                            _ => todo!(),
                        },
                        // unresolvable request.
                        Err(e) => Response::Failure(Failure::Error {
                            message: e.to_string(),
                        }),
                    })
                }),
            },
        }
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
