use super::{response, Driver, Response};
use gtk::{
    gio::Cancellable,
    glib::{gformat, Priority, Uri},
};
use std::rc::Rc;

/// Shared request interface for Gemini protocol
pub fn request_async(
    driver: &Driver,
    uri: Uri,
    cancellable: Cancellable,
    callback: impl Fn(Result<ggemini::client::Response, ggemini::client::Error>) + 'static,
) {
    driver.gemini.request_async(
        ggemini::client::Request::gemini(uri.clone()),
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
pub fn handle(
    result: Result<ggemini::client::connection::Response, ggemini::client::Error>,
    base: Uri,
    cancellable: Cancellable,
    is_source_request: bool, // @TODO yet partial implementation
    callback: Rc<impl Fn(Response) + 'static>,
) {
    use ggemini::client::connection::response::{data::Text, meta::Status};
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
