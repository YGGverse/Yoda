//! At this moment, the `Driver` contain only one protocol library,
//! by extending it features with new protocol, please make sub-module implementation

mod gemini;
pub mod status;

// Local dependencies
pub use status::Status;

// Global dependencies
use super::{
    request::{feature::Protocol, Feature},
    response,
    response::Failure,
    Request, Response,
};
use crate::{tool::now, Profile};
use gtk::{gio::SocketClientEvent, prelude::SocketClientExt};
use std::rc::Rc;

pub struct Driver {
    /// Profile reference required for Gemini protocol auth (match scope)
    profile: Rc<Profile>,
    /// Supported clients
    /// * gemini driver should be initiated once (on page object init)
    ///   to process all it connection features properly
    gemini: Rc<ggemini::Client>,
    // other clients here..
}

impl Driver {
    // Constructors

    /// Init new `Self`
    pub fn init(profile: Rc<Profile>, callback: impl Fn(Status) + 'static) -> Self {
        // Init supported protocol libraries
        let gemini = Rc::new(ggemini::Client::new());

        // Retransmit gemini [SocketClient](https://docs.gtk.org/gio/class.SocketClient.html) updates
        gemini.socket.connect_event(move |_, event, _, _| {
            callback(match event {
                SocketClientEvent::Resolving => Status::Resolving { time: now() },
                SocketClientEvent::Resolved => Status::Resolved { time: now() },
                SocketClientEvent::Connecting => Status::Connecting { time: now() },
                SocketClientEvent::Connected => Status::Connected { time: now() },
                SocketClientEvent::ProxyNegotiating => Status::ProxyNegotiating { time: now() },
                SocketClientEvent::ProxyNegotiated => Status::ProxyNegotiated { time: now() },
                // * `TlsHandshaking` | `TlsHandshaked` has effect only for guest connections!
                SocketClientEvent::TlsHandshaking => Status::TlsHandshaking { time: now() },
                SocketClientEvent::TlsHandshaked => Status::TlsHandshaked { time: now() },
                SocketClientEvent::Complete => Status::Complete { time: now() },
                _ => todo!(), // alert on API change
            })
        });

        // other client listeners here..

        // Done
        Self { profile, gemini }
    }

    // Actions

    /// Make new async `Feature` request
    /// * return `Response` in callback function
    pub fn request_async(&self, request: Request, callback: impl FnOnce(Response) + 'static) {
        let referrer = request.to_referrer();
        match request.feature {
            Feature::Download(protocol) => match protocol {
                Protocol::Gemini {
                    uri,
                    cancellable,
                    priority,
                } => gemini::request_async(
                    &self.profile,
                    &self.gemini,
                    &uri,
                    &cancellable,
                    &priority,
                    {
                        let base = uri.clone();
                        let cancellable = cancellable.clone();
                        move |result| {
                            callback(match result {
                                Ok(response) => Response::Download {
                                    base,
                                    stream: response.connection.stream(),
                                    cancellable,
                                },
                                Err(e) => Response::Failure(Failure::Error {
                                    message: e.to_string(),
                                }),
                            })
                        }
                    },
                ),
                _ => callback(Response::Failure(Failure::Error {
                    message: "Download feature yet not supported for this request".to_string(),
                })), // @TODO or maybe panic as unexpected
            },
            Feature::Default(protocol) => match protocol {
                Protocol::Gemini {
                    uri,
                    cancellable,
                    priority,
                } => gemini::request_async(
                    &self.profile,
                    &self.gemini,
                    &uri,
                    &cancellable,
                    &priority,
                    {
                        let cancellable = cancellable.clone();
                        let uri = uri.clone();

                        move |result| {
                            gemini::handle(
                                result,
                                uri,
                                cancellable,
                                priority,
                                referrer,
                                false,
                                callback,
                            )
                        }
                    },
                ),
                Protocol::Titan { .. } => todo!(),
                Protocol::Unsupported => todo!(),
            },
            Feature::Source(ref protocol) => match protocol {
                Protocol::Gemini {
                    uri,
                    cancellable,
                    priority,
                } => gemini::request_async(
                    &self.profile,
                    &self.gemini,
                    uri,
                    cancellable,
                    priority,
                    {
                        let cancellable = cancellable.clone();
                        let priority = *priority;
                        let uri = uri.clone();
                        move |result| {
                            gemini::handle(
                                result,
                                uri,
                                cancellable,
                                priority,
                                request.referrer.to_vec(),
                                true,
                                callback,
                            )
                        }
                    },
                ),
                _ => callback(Response::Failure(Failure::Error {
                    message: "Source view feature yet not supported for this request".to_string(),
                })), // @TODO or maybe panic as unexpected
            },
        }
    }
}
