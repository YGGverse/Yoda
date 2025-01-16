//! At this moment, the `Driver` contain only one protocol library,
//! by extending it features with new protocol, please make sub-module implementation

mod gemini;
mod redirect;
pub mod status;

// Local dependencies
use redirect::Redirect;
pub use status::Status;

// Global dependencies
use super::{feature::Request, response, response::Failure, Feature, Response};
use crate::{tool::now, Profile};
use gtk::{
    gio::{Cancellable, SocketClientEvent},
    prelude::SocketClientExt,
};
use std::rc::Rc;

pub struct Driver {
    /// Profile reference required for Gemini protocol auth (match request)
    profile: Rc<Profile>,
    /// Redirect resolver for different protocols
    redirect: Rc<Redirect>,
    /// Supported clients
    gemini: ggemini::Client,
    // other clients here..
}

impl Driver {
    // Constructors

    /// Init new `Self`
    pub fn init(profile: &Rc<Profile>, callback: impl Fn(Status) + 'static) -> Self {
        // Init supported protocol libraries
        let gemini = ggemini::Client::new();

        // Translate driver status to `Status`

        // Gemini
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
        Self {
            profile: profile.clone(),
            redirect: Rc::new(Redirect::new()),
            gemini,
        }
    }

    // Actions

    /// Make new async `Feature` request
    /// * return shared `Response` as the callback
    pub fn feature_async(
        &self,
        feature: Feature,
        cancellable: Cancellable,
        callback: Rc<impl Fn(Response) + 'static>,
    ) {
        match feature {
            Feature::Download { request } => match request {
                Request::Gemini { uri } => {
                    gemini::request_async(self, uri.clone(), cancellable.clone(), move |result| {
                        match result {
                            Ok(response) => callback(Response::Download {
                                base: uri.clone(),
                                stream: response.connection.stream(),
                                cancellable: cancellable.clone(),
                            }),
                            Err(e) => callback(Response::Failure(Failure::Error {
                                message: e.to_string(),
                            })),
                        }
                    })
                }
                _ => todo!(),
            },
            Feature::Default { request } => match request {
                Request::Gemini { uri } => {
                    gemini::request_async(self, uri.clone(), cancellable.clone(), move |result| {
                        gemini::handle(
                            result,
                            uri.clone(),
                            cancellable.clone(),
                            false,
                            callback.clone(),
                        )
                    })
                }
                Request::Titan { .. } => todo!(),
                Request::Undefined => todo!(),
            },
            Feature::Source { request } => match request {
                Request::Gemini { uri } => {
                    gemini::request_async(self, uri.clone(), cancellable.clone(), move |result| {
                        gemini::handle(
                            result,
                            uri.clone(),
                            cancellable.clone(),
                            true,
                            callback.clone(),
                        )
                    })
                }
                Request::Titan { .. } => todo!(),
                Request::Undefined => todo!(),
            },
        }
    }
}
