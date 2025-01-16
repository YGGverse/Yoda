//! At this moment, the `Driver` contain only one protocol library,
//! by extending it features with new protocol, please make sub-module implementation

mod redirect;
pub mod status;

// Local dependencies
use redirect::Redirect;
pub use status::Status;

// Global dependencies
use super::{feature, response, Feature, Response};
use crate::{tool::now, Profile};
use gtk::{
    gio::{Cancellable, SocketClientEvent},
    glib::{gformat, Priority, Uri},
    prelude::SocketClientExt,
};
use std::rc::Rc;

pub struct Driver {
    /// Profile reference required for Gemini protocol auth (match request)
    profile: Rc<Profile>,
    /// Redirect resolver for different protocols
    redirect: Rc<Redirect>,
    /// Supported clients
    gemini: gemini::Client,
    // other clients here..
}

impl Driver {
    pub fn init(profile: &Rc<Profile>, callback: impl Fn(Status) + 'static) -> Self {
        // Init protocol driver libraries
        let gemini = gemini::Client::new();

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

    pub fn feature_async(
        &self,
        feature: Feature,
        cancellable: Cancellable,
        callback: Rc<impl Fn(Response) + 'static>,
    ) {
        match feature {
            Feature::Download { request } => match request {
                feature::Request::Gemini { uri } => {
                    request_gemini_async(self, uri.clone(), cancellable.clone(), move |result| {
                        match result {
                            Ok(response) => callback(Response::Download {
                                base: uri.clone(),
                                stream: response.connection.stream(),
                                cancellable: cancellable.clone(),
                            }),
                            Err(e) => callback(Response::Failure(response::Failure::Error {
                                message: e.to_string(),
                            })),
                        }
                    })
                }
                _ => todo!(),
            },
            Feature::Default { request } => match request {
                feature::Request::Gemini { uri } => {
                    request_gemini_async(self, uri.clone(), cancellable.clone(), move |result| {
                        handle_gemini(
                            result,
                            uri.clone(),
                            cancellable.clone(),
                            false,
                            callback.clone(),
                        )
                    })
                }
                feature::Request::Titan { .. } => todo!(),
                feature::Request::Undefined => todo!(),
            },
            Feature::Source { request } => match request {
                feature::Request::Gemini { uri } => {
                    request_gemini_async(self, uri.clone(), cancellable.clone(), move |result| {
                        handle_gemini(
                            result,
                            uri.clone(),
                            cancellable.clone(),
                            true,
                            callback.clone(),
                        )
                    })
                }
                feature::Request::Titan { .. } => todo!(),
                feature::Request::Undefined => todo!(),
            },
        }
    }
}

/// Shared request interface for Gemini protocol
fn request_gemini_async(
    driver: &Driver,
    uri: Uri,
    cancellable: Cancellable,
    callback: impl Fn(Result<gemini::client::Response, gemini::client::Error>) + 'static,
) {
    driver.gemini.request_async(
        gemini::client::Request::gemini(uri.clone()),
        Priority::DEFAULT,
        cancellable.clone(),
        // Search for user certificate match request
        // * @TODO this feature does not support multi-protocol yet
        match driver.profile.identity.gemini.match_scope(&uri.to_string()) {
            Some(identity) => match identity.to_tls_certificate() {
                Ok(certificate) => Some(certificate),
                Err(_) => todo!(),
            },
            None => None,
        },
        move |result| callback(result),
    )
}

/// Shared handler for Gemini `Result`
/// * same implementation for Gemini and Titan protocols response
fn handle_gemini(
    result: Result<gemini::client::connection::Response, gemini::client::Error>,
    base: Uri,
    cancellable: Cancellable,
    is_source_request: bool, // @TODO yet partial implementation
    callback: Rc<impl Fn(Response) + 'static>,
) {
    use gemini::client::connection::response::{data::Text, meta::Status};
    match result {
        Ok(response) => match response.meta.status {
            // https://geminiprotocol.net/docs/protocol-specification.gmi#input-expected
            Status::Input => callback(Response::Input(response::Input::Response {
                base,
                title: match response.meta.data {
                    Some(data) => data.value,
                    None => gformat!("Input expected"),
                },
            })),
            Status::SensitiveInput => callback(Response::Input(response::Input::Sensitive {
                base,
                title: match response.meta.data {
                    Some(data) => data.value,
                    None => gformat!("Input expected"),
                },
            })),
            // https://geminiprotocol.net/docs/protocol-specification.gmi#status-20
            Status::Success => {
                let mime = response.meta.mime.unwrap().value.to_lowercase();
                match mime.as_str() {
                    "text/gemini" => Text::from_stream_async(
                        response.connection.stream(),
                        Priority::DEFAULT,
                        cancellable,
                        move |result| match result {
                            Ok(text) => callback(Response::TextGemini {
                                base,
                                source: text.data,
                                is_source_request,
                            }),
                            Err(_) => todo!(),
                        },
                    ),
                    "image/png" | "image/gif" | "image/jpeg" | "image/webp" => {
                        callback(Response::Stream {
                            base,
                            mime,
                            stream: response.connection.stream(),
                            cancellable,
                        })
                    }
                    mime => callback(Response::Failure(response::Failure::Mime {
                        message: format!("Undefined content type `{mime}`"),
                    })),
                } // @TODO handle `None`
            }
            // https://geminiprotocol.net/docs/protocol-specification.gmi#status-30-temporary-redirection
            // https://geminiprotocol.net/docs/protocol-specification.gmi#status-31-permanent-redirection
            Status::Redirect | Status::PermanentRedirect => todo!(),
            // https://geminiprotocol.net/docs/protocol-specification.gmi#status-60
            Status::CertificateRequest => {
                callback(Response::Certificate(response::Certificate::Request {
                    title: match response.meta.data {
                        Some(data) => data.value,
                        None => gformat!("Client certificate required"),
                    },
                }))
            }
            // https://geminiprotocol.net/docs/protocol-specification.gmi#status-61-certificate-not-authorized
            Status::CertificateUnauthorized => {
                callback(Response::Certificate(response::Certificate::Request {
                    title: match response.meta.data {
                        Some(data) => data.value,
                        None => gformat!("Certificate not authorized"),
                    },
                }))
            }
            // https://geminiprotocol.net/docs/protocol-specification.gmi#status-62-certificate-not-valid
            Status::CertificateInvalid => {
                callback(Response::Certificate(response::Certificate::Request {
                    title: match response.meta.data {
                        Some(data) => data.value,
                        None => gformat!("Certificate not valid"),
                    },
                }))
            }
            status => callback(Response::Failure(response::Failure::Status {
                message: format!("Undefined status code `{:?}`", status), // @TODO implement display trait for `ggemini` lib
            })),
        },
        Err(e) => callback(Response::Failure(response::Failure::Error {
            message: e.to_string(),
        })),
    }
}
