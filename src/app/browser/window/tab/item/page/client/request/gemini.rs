use super::{super::response::*, Client, Feature, Request, Response};

use gtk::{
    gio::Cancellable,
    glib::{Priority, Uri, UriFlags},
};

pub fn send(
    client: &Client,
    feature: Feature,
    uri: Uri,
    referrer: Vec<Request>,
    cancellable: Cancellable,
    callback: impl FnOnce(Response) + 'static,
) {
    request(
        client,
        uri.clone(),
        cancellable.clone(),
        move |result| match result {
            Ok(response) => handle(response, uri, referrer, feature, cancellable, callback),
            Err(e) => callback(Response::Failure(Failure::Error {
                message: e.to_string(),
            })),
        },
    )
}

/// Shared request interface for Gemini protocol
fn request(
    client: &Client,
    uri: Uri,
    cancellable: Cancellable,
    callback: impl FnOnce(Result<ggemini::client::Response, ggemini::client::Error>) + 'static,
) {
    let request = uri.to_string();
    client.gemini.request_async(
        ggemini::client::Request::gemini(uri.clone()),
        Priority::DEFAULT,
        cancellable.clone(),
        // Search for user certificate match request
        // * @TODO this feature does not support multi-protocol yet
        match client.profile.identity.gemini.match_scope(&request) {
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
fn handle(
    response: ggemini::client::connection::Response,
    base: Uri,
    referrer: Vec<Request>,
    feature: Feature,
    cancellable: Cancellable,
    callback: impl FnOnce(Response) + 'static,
) {
    use ggemini::client::connection::response::{data::Text, meta::Status};
    match response.meta.status {
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
                    Priority::DEFAULT,
                    cancellable.clone(),
                    move |result| match result {
                        Ok(text) => callback(Response::TextGemini {
                            base,
                            source: text.data,
                            is_source_request: matches!(feature, Feature::Source),
                        }),
                        Err(e) => callback(Response::Failure(Failure::Mime {
                            base,
                            mime: mime.to_string(),
                            message: e.to_string(),
                        })),
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
        Status::Redirect => callback(redirect(
            response,
            base,
            referrer,
            cancellable,
            Priority::DEFAULT,
            false,
        )),
        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-31-permanent-redirection
        Status::PermanentRedirect => callback(redirect(
            response,
            base,
            referrer,
            cancellable,
            Priority::DEFAULT,
            true,
        )),
        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-60
        Status::CertificateRequest => callback(Response::Certificate(Certificate::Request {
            title: match response.meta.data {
                Some(data) => data.to_gstring(),
                None => "Client certificate required".into(),
            },
        })),
        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-61-certificate-not-authorized
        Status::CertificateUnauthorized => callback(Response::Certificate(Certificate::Request {
            title: match response.meta.data {
                Some(data) => data.to_gstring(),
                None => "Certificate not authorized".into(),
            },
        })),
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
    }
}

/// Shared redirection `Response` builder
fn redirect(
    // Subject to parse
    response: ggemini::client::connection::Response,
    // Wanted to process relative links
    base: Uri,
    // List of previous requests to handle redirection rules
    referrer: Vec<Request>,
    cancellable: Cancellable,
    priority: Priority,
    is_foreground: bool,
) -> Response {
    // Validate redirection attempt
    // [Gemini protocol specifications](https://geminiprotocol.net/docs/protocol-specification.gmi#redirection)
    if referrer.len() > 5 {
        return Response::Failure(Failure::Error {
            message: "Max redirection count reached".to_string(),
        });
    }
    // Target URL expected from client response meta data
    match response.meta.data {
        Some(target) => match Uri::parse_relative(&base, target.as_str(), UriFlags::NONE) {
            Ok(target) => {
                // Disallow external redirection
                if base.scheme() != target.scheme()
                    || base.port() != target.port()
                    || base.host() != target.host()
                {
                    return Response::Failure(Failure::Error {
                        message: "External redirects not allowed by protocol specification"
                            .to_string(),
                    }); // @TODO placeholder page with optional link open button
                }

                // Build new `Request` for redirection `Response`
                // * make sure that `referrer` already contain current `Request`
                //  (to validate redirection count in chain)
                todo!()
                /*let request =
                    Request::build(&target.to_string(), Some(referrer), cancellable, priority);

                Response::Redirect(if is_foreground {
                    Redirect::Foreground(request)
                } else {
                    Redirect::Background(request)
                })*/
            }
            Err(e) => Response::Failure(Failure::Error {
                message: format!("Could not parse target address: {e}"),
            }),
        },
        None => Response::Failure(Failure::Error {
            message: "Target address not found".to_string(),
        }),
    }
}
