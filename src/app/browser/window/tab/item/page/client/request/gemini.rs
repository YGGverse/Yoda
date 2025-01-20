use super::{super::response::*, Client, Feature, Response};

use gtk::{
    gio::Cancellable,
    glib::{Priority, Uri, UriFlags},
};

pub struct Gemini {
    pub referrer: Option<Box<Self>>,
    pub uri: Uri,
}

impl Gemini {
    // Actions

    pub fn handle(
        self,
        client: &Client,
        cancellable: Cancellable,
        callback: impl FnOnce(Response) + 'static,
    ) {
        use ggemini::client::connection::response::{data::Text, meta::Status};

        client.gemini.request_async(
            ggemini::client::Request::gemini(self.uri.clone()),
            Priority::DEFAULT,
            cancellable.clone(),
            // Search for user certificate match request
            // * @TODO this feature does not support multi-protocol yet
            match client
                .profile
                .identity
                .gemini
                .match_scope(&self.uri.to_string())
            {
                Some(identity) => match identity.to_tls_certificate() {
                    Ok(certificate) => Some(certificate),
                    Err(_) => panic!(), // unexpected
                },
                None => None,
            },
            |result| match result {
                Ok(response) => {
                    match response.meta.status {
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#input-expected
                        Status::Input => callback(Response::Input(Input::Response {
                            base: self.uri.clone(),
                            title: match response.meta.data {
                                Some(data) => data.to_gstring(),
                                None => "Input expected".into(),
                            },
                        })),
                        Status::SensitiveInput => callback(Response::Input(Input::Sensitive {
                            base: self.uri.clone(),
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
                                        Ok(text) => callback(Response::Text(
                                            super::super::response::Text::Gemini {
                                                base: self.uri.clone(),
                                                data: text.to_string(),
                                            },
                                        )),
                                        Err(e) => callback(Response::Failure(Failure::Mime {
                                            base: self.uri.clone(),
                                            mime: mime.to_string(),
                                            message: e.to_string(),
                                        })),
                                    },
                                ),
                                "image/png" | "image/gif" | "image/jpeg" | "image/webp" => {
                                    callback(Response::Stream {
                                        base: self.uri.clone(),
                                        mime: mime.to_string(),
                                        stream: response.connection.stream(),
                                        cancellable,
                                    })
                                }
                                mime => callback(Response::Failure(Failure::Mime {
                                    base: self.uri.clone(),
                                    mime: mime.to_string(),
                                    message: format!("Content type `{mime}` yet not supported"),
                                })),
                            },
                            None => callback(Response::Failure(Failure::Error {
                                message: "MIME type not found".to_string(),
                            })),
                        },
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-30-temporary-redirection
                        Status::Redirect => callback(self.redirect(response, false)),
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-31-permanent-redirection
                        Status::PermanentRedirect => callback(self.redirect(response, true)),
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-60
                        Status::CertificateRequest => {
                            callback(Response::Certificate(Certificate::Request {
                                title: match response.meta.data {
                                    Some(data) => data.to_gstring(),
                                    None => "Client certificate required".into(),
                                },
                            }))
                        }
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
                        Status::CertificateInvalid => {
                            callback(Response::Certificate(Certificate::Request {
                                title: match response.meta.data {
                                    Some(data) => data.to_gstring(),
                                    None => "Certificate not valid".into(),
                                },
                            }))
                        }
                        status => callback(Response::Failure(Failure::Status {
                            message: format!("Undefined status code `{status}`"),
                        })),
                    }
                }
                Err(e) => callback(Response::Failure(Failure::Error {
                    message: e.to_string(),
                })),
            },
        )
    }

    /// Redirection builder for `Self`
    /// * [Redirect specification](https://geminiprotocol.net/docs/protocol-specification.gmi#redirection)
    fn redirect(
        self,
        response: ggemini::client::connection::Response,
        is_permanent: bool,
    ) -> Response {
        // Validate redirection count
        if self.referrers() > 5 {
            return Response::Failure(Failure::Error {
                message: "Max redirection count reached".to_string(),
            });
        }

        // Target URL expected from response meta data
        match response.meta.data {
            Some(target) => {
                match Uri::parse_relative(&self.uri, target.as_str(), UriFlags::NONE) {
                    Ok(target) => {
                        // Disallow external redirection
                        if self.uri.scheme() != target.scheme()
                            || self.uri.port() != target.port()
                            || self.uri.host() != target.host()
                        {
                            return Response::Failure(Failure::Error {
                                message: "External redirects not allowed by protocol specification"
                                    .to_string(),
                            }); // @TODO placeholder page with optional link open button
                        }
                        // Build new request
                        Response::Redirect(if is_permanent {
                            Redirect::Foreground(target)
                        } else {
                            Redirect::Background(target)
                        })
                    }
                    Err(e) => Response::Failure(Failure::Error {
                        message: e.to_string(),
                    }),
                }
            }
            None => Response::Failure(Failure::Error {
                message: "Target address not found".to_string(),
            }),
        }
    }

    /// Recursively count referrers of `Self`
    /// * useful to apply redirection rules by protocol driver selected
    pub fn referrers(&self) -> usize {
        self.referrer
            .as_ref()
            .map_or(0, |request| request.referrers())
            + 1
    }
}

/* @TODO

#[test]
fn test_referrers() {
    const QUERY: &str = "gemini://geminiprotocol.net";

    let r1 = Request::parse(QUERY, None).unwrap();
    let r2 = Request::parse(QUERY, Some(r1)).unwrap();
    let r3 = Request::parse(QUERY, Some(r2)).unwrap();

    assert_eq!(r3.referrers(), 3);
}

*/
