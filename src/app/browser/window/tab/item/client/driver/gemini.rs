use super::{Feature, Page};
use ggemini::client::{
    connection::response::{data::Text, meta::Status},
    Client, Request,
};
use gtk::glib::Bytes;
use gtk::glib::{GString, UriFlags};
use gtk::{
    gdk::Texture,
    gdk_pixbuf::Pixbuf,
    gio::{Cancellable, SocketClientEvent},
    glib::{Priority, Uri},
    prelude::{FileExt, SocketClientExt},
};
use std::{cell::Cell, path::MAIN_SEPARATOR, rc::Rc, time::Duration};

/// Multi-protocol client API for `Page` object
pub struct Gemini {
    /// Should be initiated once
    client: Rc<Client>,
    /// Validate redirection count by Gemini protocol specification
    redirects: Rc<Cell<usize>>,
    /// Handle target
    page: Rc<Page>,
}

impl Gemini {
    // Constructors

    /// Create new `Self`
    pub fn init(page: &Rc<Page>) -> Self {
        // Init supported protocol libraries
        let client = Rc::new(ggemini::Client::new());

        // Listen for [SocketClient](https://docs.gtk.org/gio/class.SocketClient.html) updates
        client.socket.connect_event({
            let page = page.clone();
            move |_, event, _, _| {
                page.set_progress(match event {
                    // 0.1 reserved for handle begin
                    SocketClientEvent::Resolving => 0.2,
                    SocketClientEvent::Resolved => 0.3,
                    SocketClientEvent::Connecting => 0.4,
                    SocketClientEvent::Connected => 0.5,
                    SocketClientEvent::ProxyNegotiating => 0.6,
                    SocketClientEvent::ProxyNegotiated => 0.7,
                    // * `TlsHandshaking` | `TlsHandshaked` has effect only for guest connections!
                    SocketClientEvent::TlsHandshaking => 0.8,
                    SocketClientEvent::TlsHandshaked => 0.9,
                    SocketClientEvent::Complete => 1.0,
                    _ => todo!(), // alert on API change
                })
            }
        });

        Self {
            client,
            redirects: Rc::new(Cell::new(0)),
            page: page.clone(),
        }
    }

    // Actions

    pub fn handle(&self, uri: Uri, feature: Rc<Feature>, cancellable: Cancellable) {
        use ggemini::client::connection::Request;

        match uri.scheme().as_str() {
            "gemini" => handle(
                Request::Gemini { uri },
                self.client.clone(),
                self.page.clone(),
                self.redirects.clone(),
                feature,
                cancellable,
                None,
            ),
            "titan" => {
                self.page.input.set_new_titan({
                    let client = self.client.clone();
                    let page = self.page.clone();
                    let redirects = self.redirects.clone();
                    move |data, on_failure| {
                        handle(
                            Request::Titan {
                                uri: uri.clone(),
                                data: Bytes::from(data),
                                // * some servers may reject the request without content type
                                mime: Some("text/plain".to_string()),
                                token: None, // @TODO
                            },
                            client.clone(),
                            page.clone(),
                            redirects.clone(),
                            feature.clone(),
                            cancellable.clone(),
                            Some(on_failure),
                        )
                    }
                });
                self.page.set_title("Titan input");
                self.page.set_progress(0.0);
            }
            _ => panic!(), // unexpected
        }
    }
}

fn handle(
    request: Request,
    client: Rc<Client>,
    page: Rc<Page>,
    redirects: Rc<Cell<usize>>,
    feature: Rc<Feature>,
    cancellable: Cancellable,
    on_failure: Option<Box<dyn Fn()>>,
) {
    let uri = request.uri().clone();
    client.request_async(
        request,
        Priority::DEFAULT,
        cancellable.clone(),
        // Search for user certificate match request
        // * @TODO this feature does not support multi-protocol yet
        match page
            .profile
            .identity
            .get(&uri.to_string())
        {
            Some(identity) => match identity.to_tls_certificate() {
                Ok(certificate) => Some(certificate),
                Err(_) => panic!(), // unexpected
            },
            None => None,
        },
        {
            let page = page.clone();
            let redirects = redirects.clone();
            move |result| match result {
                Ok(response) => {
                    match response.meta.status {
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#input-expected
                        Status::Input | Status::SensitiveInput => {
                            let title = match response.meta.data {
                                Some(data) => data.to_string(),
                                None => Status::Input.to_string(),
                            };
                            if matches!(response.meta.status, Status::SensitiveInput) {
                                page.input.set_new_sensitive(
                                    page.item_action.clone(),
                                    uri,
                                    Some(&title),
                                    Some(1024),
                                );
                            } else {
                                page.input.set_new_response(
                                    page.item_action.clone(),
                                    uri,
                                    Some(&title),
                                    Some(1024),
                                );
                            }
                            page.set_progress(0.0);
                            page.set_title(&title);
                            redirects.replace(0); // reset
                        }
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-20
                        Status::Success => match *feature {
                            Feature::Download => {
                                // Init download widget
                                let status = page.content.to_status_download(
                                    uri_to_title(&uri).trim_matches(MAIN_SEPARATOR), // grab default filename from base URI,
                                    // format FS entities
                                    &cancellable,
                                    {
                                        let cancellable = cancellable.clone();
                                        let stream = response.connection.stream();
                                        move |file, action| {
                                            match file.replace(
                                                None,
                                                false,
                                                gtk::gio::FileCreateFlags::NONE,
                                                Some(&cancellable),
                                            ) {
                                                Ok(file_output_stream) => {
                                                    use crate::tool::Format;
                                                    // Asynchronously read [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
                                                    // to local [MemoryInputStream](https://docs.gtk.org/gio/class.MemoryInputStream.html)
                                                    // show bytes count in loading widget, validate max size for incoming data
                                                    // * no dependency of Gemini library here, feel free to use any other `IOStream` processor
                                                    ggemini::gio::file_output_stream::move_all_from_stream_async(
                                                        stream.clone(),
                                                        file_output_stream,
                                                        cancellable.clone(),
                                                        Priority::DEFAULT,
                                                        (
                                                            0x100000, // 1M bytes per chunk
                                                            None,     // unlimited
                                                            0,        // initial totals
                                                        ),
                                                        (
                                                            // on chunk
                                                            {
                                                                let action = action.clone();
                                                                move |_, total| action.update.activate(&format!(
                                                                    "Received {}...",
                                                                    total.bytes()
                                                                ))
                                                            },
                                                            // on complete
                                                            {
                                                                let action = action.clone();
                                                                move |result| match result {
                                                                    Ok((_, total)) => {
                                                                        action.complete.activate(&format!(
                                                                            "Saved to {} ({} total)",
                                                                            file.parse_name(),
                                                                            total.bytes()
                                                                        ))
                                                                    }
                                                                    Err(e) => action.cancel.activate(&e.to_string())
                                                                }
                                                            },
                                                        ),
                                                    )
                                                }
                                                Err(e) => action.cancel.activate(&e.to_string()),
                                            }
                                        }
                                    },
                                );
                                page.set_progress(0.0);
                                page.set_title(&status.title());
                                redirects.replace(0); // reset
                            },
                            _ => match response.meta.mime {
                                Some(mime) => match mime.as_str() {
                                    "text/gemini" => Text::from_stream_async(
                                        response.connection.stream(),
                                        Priority::DEFAULT,
                                        cancellable.clone(),
                                        move |result| match result {
                                            Ok(text) => {
                                                let widget = if matches!(*feature, Feature::Source) {
                                                    page.content.to_text_source(&text.to_string())
                                                } else {
                                                    page.content.to_text_gemini(&uri, &text.to_string())
                                                };
                                                page.search.set(Some(widget.text_view));
                                                page.set_title(&match widget.meta.title {
                                                    Some(title) => title.into(), // @TODO
                                                    None => uri_to_title(&uri),
                                                });
                                                page.set_progress(0.0);
                                                page.window_action
                                                    .find
                                                    .simple_action
                                                    .set_enabled(true);
                                                redirects.replace(0); // reset
                                            }
                                            Err(e) => {
                                                let status = page.content.to_status_failure();
                                                status.set_description(Some(&e.to_string()));
                                                page.set_progress(0.0);
                                                page.set_title(&status.title());
                                                redirects.replace(0); // reset
                                            },
                                        },
                                    ),
                                    "image/png" | "image/gif" | "image/jpeg" | "image/webp" => {
                                        // Final image size unknown, show loading widget
                                        let status = page.content.to_status_loading(
                                            Some(Duration::from_secs(1)), // show if download time > 1 second
                                        );

                                        // Asynchronously read [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
                                        // to local [MemoryInputStream](https://docs.gtk.org/gio/class.MemoryInputStream.html)
                                        // show bytes count in loading widget, validate max size for incoming data
                                        // * no dependency of Gemini library here, feel free to use any other `IOStream` processor
                                        ggemini::gio::memory_input_stream::from_stream_async(
                                            response.connection.stream(),
                                            cancellable.clone(),
                                            Priority::DEFAULT,
                                            0x400, // 1024 bytes per chunk, optional step for images download tracking
                                            0xA00000, // 10M bytes max to prevent memory overflow if server play with promises
                                            move |_, total|
                                            status.set_description(Some(&format!("Download: {total} bytes"))),
                                            {
                                                let page = page.clone();
                                                move |result| match result {
                                                    Ok((memory_input_stream, _)) => {
                                                        Pixbuf::from_stream_async(
                                                            &memory_input_stream,
                                                            Some(&cancellable),
                                                            move |result| {
                                                                match result {
                                                                    Ok(buffer) => {
                                                                        page.set_title(&uri_to_title(&uri));
                                                                        page.content.to_image(&Texture::for_pixbuf(&buffer));
                                                                    }
                                                                    Err(e) => {
                                                                        let status = page.content.to_status_failure();
                                                                        status.set_description(Some(e.message()));
                                                                        page.set_title(&status.title());
                                                                    }
                                                                }
                                                                page.set_progress(0.0);
                                                                redirects.replace(0); // reset
                                                            },
                                                        )
                                                    }
                                                    Err(e) => {
                                                        let status = page.content.to_status_failure();
                                                        status.set_description(Some(&e.to_string()));
                                                        page.set_progress(0.0);
                                                        page.set_title(&status.title());
                                                        redirects.replace(0); // reset
                                                    }
                                                }
                                            },
                                        )
                                    }
                                    mime => {
                                        let status = page
                                            .content
                                            .to_status_mime(mime, Some((&page.item_action, &uri)));
                                        status.set_description(Some(&format!("Content type `{mime}` yet not supported")));
                                        page.set_progress(0.0);
                                        page.set_title(&status.title());
                                        redirects.replace(0); // reset
                                    },
                                },
                                None => {
                                    let status = page.content.to_status_failure();
                                    status.set_description(Some("MIME type not found"));
                                    page.set_progress(0.0);
                                    page.set_title(&status.title());
                                    redirects.replace(0); // reset
                                },
                            }
                        },
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-30-temporary-redirection
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-31-permanent-redirection
                        Status::PermanentRedirect | Status::Redirect => {
                            // Expected target URL in response meta
                            match response.meta.data {
                                Some(data) => match uri.parse_relative(data.as_str(), UriFlags::NONE) {
                                    Ok(absolute) => {
                                        // Base donor scheme could be `titan`, rewrite new relative links resolved with `gemini`
                                        // otherwise, keep original scheme to handle external redirect rules properly
                                        // * in this case, `titan` scheme redirects unexpected
                                        let scheme = absolute.scheme();
                                        let target = Uri::build(
                                            UriFlags::NONE,
                                            &if "titan" == scheme {
                                                scheme.replace("titan", "gemini")
                                            } else {
                                                scheme.to_string()
                                            },
                                            absolute.userinfo().as_deref(),
                                            absolute.host().as_deref(),
                                            absolute.port(),
                                            absolute.path().as_str(),
                                            absolute.query().as_deref(),
                                            absolute.fragment().as_deref(),
                                        );
                                        // Increase client redirection counter
                                        let total = redirects.take() + 1;
                                        // Validate total redirects by protocol specification
                                        if total > 5 {
                                            let status = page.content.to_status_failure();
                                            status.set_description(Some("Redirection limit reached"));
                                            page.set_progress(0.0);
                                            page.set_title(&status.title());
                                            redirects.replace(0); // reset
                                        /* @TODO can't find that in specification:
                                        // Disallow external redirection by protocol restrictions
                                        } else if "gemini" != target.scheme()
                                            || uri.port() != target.port()
                                            || uri.host() != target.host() {
                                                let status = page.content.to_status_failure();
                                                status.set_description(Some("External redirects not allowed by protocol specification"));
                                                page.set_progress(0.0);
                                                page.set_title(&status.title());
                                                redirects.replace(0); // reset
                                        */
                                        // Valid
                                        } else {
                                            if matches!(response.meta.status, Status::PermanentRedirect) {
                                                page.navigation.set_request(&uri.to_string());
                                            }
                                            redirects.replace(total);
                                            page.item_action.load.activate(Some(&target.to_string()), false);
                                        }
                                    }
                                    Err(e) => {
                                        let status = page.content.to_status_failure();
                                        status.set_description(Some(&e.to_string()));
                                        page.set_progress(0.0);
                                        page.set_title(&status.title());
                                        redirects.replace(0); // reset
                                    }
                                }
                                None => {
                                    let status = page.content.to_status_failure();
                                    status.set_description(Some("Redirection target not found"));
                                    page.set_progress(0.0);
                                    page.set_title(&status.title());
                                    redirects.replace(0); // reset
                                }
                            }
                        },
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-60
                        Status::CertificateRequest |
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-61-certificate-not-authorized
                        Status::CertificateUnauthorized |
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-62-certificate-not-valid
                        Status::CertificateInvalid => {
                            let status = page.content.to_status_identity();
                            status.set_description(Some(&match response.meta.data {
                                Some(data) => data.to_string(),
                                None => response.meta.status.to_string(),
                            }));

                            page.set_progress(0.0);
                            page.set_title(&status.title());
                            redirects.replace(0); // reset
                        }
                        error => {
                            let status = page.content.to_status_failure();

                            status.set_description(
                                Some(&match response.meta.data {
                                    Some(message) => message.to_string(),
                                    None => error.to_string()
                                })
                            );
                            page.set_progress(0.0);
                            page.set_title(&status.title());
                            redirects.replace(0); // reset

                            if let Some(callback) = on_failure {
                                callback()
                            }
                        },
                    }
                }
                Err(e) => {
                    let status = page.content.to_status_failure();
                    status.set_description(Some(&e.to_string()));
                    page.set_progress(0.0);
                    page.set_title(&status.title());
                    redirects.replace(0); // reset
                }
            }
        },
    )
}

/// Helper function, extract readable title from [Uri](https://docs.gtk.org/glib/struct.Uri.html)
/// * useful as common placeholder when page title could not be detected
/// * this feature may be improved and moved outside @TODO
fn uri_to_title(uri: &Uri) -> GString {
    let path = uri.path();
    if path.split('/').last().unwrap_or_default().is_empty() {
        match uri.host() {
            Some(host) => host,
            None => "Untitled".into(),
        }
    } else {
        path
    }
}
