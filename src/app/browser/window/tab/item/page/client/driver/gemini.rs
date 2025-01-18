use super::{
    response::{Certificate, Failure, Input, Redirect},
    Profile, Response,
};
use gtk::{
    gio::Cancellable,
    glib::{Priority, Uri, UriFlags},
};
use std::rc::Rc;

/// Shared request interface for Gemini protocol
pub fn request_async(
    profile: &Rc<Profile>,
    client: &Rc<ggemini::Client>,
    uri: Uri,
    cancellable: Cancellable,
    priority: Priority,
    callback: impl FnOnce(Result<ggemini::client::Response, ggemini::client::Error>) + 'static,
) {
    let request = uri.to_string();
    client.request_async(
        ggemini::client::Request::gemini(uri),
        priority,
        cancellable,
        // Search for user certificate match request
        // * @TODO this feature does not support multi-protocol yet
        match profile.identity.gemini.match_scope(&request) {
            Some(identity) => match identity.to_tls_certificate() {
                Ok(certificate) => Some(certificate),
                Err(_) => todo!(),
            },
            None => None,
        },
        callback,
    )
}

/// Shared handler for Gemini `Result`
/// * same implementation for Gemini and Titan protocols response
pub fn handle(
    result: Result<ggemini::client::connection::Response, ggemini::client::Error>,
    base: Uri,
    cancellable: Cancellable,
    priority: Priority,
    is_source_request: bool, // @TODO yet partial implementation
    callback: impl FnOnce(Response) + 'static,
) {
    use ggemini::client::connection::response::{data::Text, meta::Status};
    match result {
        Ok(response) => match response.meta.status {
            // https://geminiprotocol.net/docs/protocol-specification.gmi#input-expected
            Status::Input => callback(Response::Input(Input::Response {
                base,
                title: match response.meta.data {
                    Some(data) => data.to_gstring(),
                    None => "Input expected".into(),
                },
            })),
            Status::SensitiveInput => callback(Response::Input(Input::Sensitive {
                base,
                title: match response.meta.data {
                    Some(data) => data.to_gstring(),
                    None => "Input expected".into(),
                },
            })),
            // https://geminiprotocol.net/docs/protocol-specification.gmi#status-20
            Status::Success => match response.meta.mime {
                Some(mime) => match mime.as_str() {
                    "text/gemini" => Text::from_stream_async(
                        response.connection.stream(),
                        priority,
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
                            mime: mime.to_string(),
                            stream: response.connection.stream(),
                            cancellable,
                        })
                    }
                    mime => callback(Response::Failure(Failure::Mime {
                        base,
                        mime: mime.to_string(),
                        message: format!("Content type `{mime}` yet not supported"),
                    })),
                },
                None => callback(Response::Failure(Failure::Error {
                    message: "MIME type not found".to_string(),
                })),
            },
            // https://geminiprotocol.net/docs/protocol-specification.gmi#status-30-temporary-redirection
            Status::Redirect => callback(match response.meta.data {
                Some(data) => match Uri::parse_relative(&base, data.as_str(), UriFlags::NONE) {
                    Ok(target) => Response::Redirect(Redirect::Foreground {
                        source: base,
                        target,
                    }),
                    Err(e) => Response::Failure(Failure::Error {
                        message: format!("Could not parse target address: {e}"),
                    }),
                },
                None => Response::Failure(Failure::Error {
                    message: "Target address not found".to_string(),
                }),
            }), // @TODO validate redirect count
            // https://geminiprotocol.net/docs/protocol-specification.gmi#status-31-permanent-redirection
            Status::PermanentRedirect => callback(match response.meta.data {
                Some(data) => match Uri::parse_relative(&base, data.as_str(), UriFlags::NONE) {
                    Ok(target) => Response::Redirect(Redirect::Background {
                        source: base,
                        target,
                    }),
                    Err(e) => Response::Failure(Failure::Error {
                        message: format!("Could not parse target address: {e}"),
                    }),
                },
                None => Response::Failure(Failure::Error {
                    message: "Target address not found".to_string(),
                }),
            }), // @TODO validate redirect count
            // https://geminiprotocol.net/docs/protocol-specification.gmi#status-60
            Status::CertificateRequest => callback(Response::Certificate(Certificate::Request {
                title: match response.meta.data {
                    Some(data) => data.to_gstring(),
                    None => "Client certificate required".into(),
                },
            })),
            // https://geminiprotocol.net/docs/protocol-specification.gmi#status-61-certificate-not-authorized
            Status::CertificateUnauthorized => {
                callback(Response::Certificate(Certificate::Request {
                    title: match response.meta.data {
                        Some(data) => data.to_gstring(),
                        None => "Certificate not authorized".into(),
                    },
                }))
            }
            // https://geminiprotocol.net/docs/protocol-specification.gmi#status-62-certificate-not-valid
            Status::CertificateInvalid => callback(Response::Certificate(Certificate::Request {
                title: match response.meta.data {
                    Some(data) => data.to_gstring(),
                    None => "Certificate not valid".into(),
                },
            })),
            status => callback(Response::Failure(Failure::Status {
                message: format!("Undefined status code `{status}`"),
            })),
        },
        Err(e) => callback(Response::Failure(Failure::Error {
            message: e.to_string(),
        })),
    }
}
