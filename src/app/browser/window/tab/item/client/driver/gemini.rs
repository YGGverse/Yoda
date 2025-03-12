use super::{Feature, Page};
use ggemini::client::connection::response::{
    failure::{Permanent, Temporary},
    Certificate, Failure, Input, Redirect,
};
use ggemini::{
    client::{Client, Request, Response},
    gio::{file_output_stream, memory_input_stream},
};
use gtk::{
    gdk::Texture,
    gdk_pixbuf::Pixbuf,
    gio::{Cancellable, SocketClientEvent},
    glib::{Priority, Uri},
    prelude::{FileExt, SocketClientExt},
};
use sourceview::prelude::InputStreamExtManual;
use std::{cell::Cell, path::MAIN_SEPARATOR, rc::Rc, time::Duration};

/// [Gemini protocol](https://geminiprotocol.net/docs/protocol-specification.gmi) client driver
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

    pub fn handle(
        &self,
        uri: Uri,
        feature: Rc<Feature>,
        cancellable: Cancellable,
        is_snap_history: bool,
    ) {
        use ggemini::client::connection::Request;

        match uri.scheme().as_str() {
            "gemini" => handle(
                Request::Gemini { uri },
                (
                    self.client.clone(),
                    self.page.clone(),
                    self.redirects.clone(),
                    feature,
                ),
                cancellable,
                None,
                is_snap_history,
            ),
            "titan" => {
                self.page.input.set_new_titan({
                    let client = self.client.clone();
                    let page = self.page.clone();
                    let redirects = self.redirects.clone();
                    move |header, bytes, on_failure| {
                        handle(
                            Request::Titan {
                                uri: uri.clone(),
                                data: bytes,
                                mime: header.mime.map(|mime| mime.into()),
                                token: header.token.map(|token| token.into()),
                            },
                            (
                                client.clone(),
                                page.clone(),
                                redirects.clone(),
                                feature.clone(),
                            ),
                            cancellable.clone(),
                            Some(on_failure),
                            is_snap_history,
                        )
                    }
                });
                self.page.set_title("Titan input");
                self.page.set_progress(0.0);
                //self.page.snap_history();
            }
            _ => panic!(), // unexpected
        }
    }
}

fn handle(
    request: Request,
    (client, page, redirects, feature): (Rc<Client>, Rc<Page>, Rc<Cell<usize>>, Rc<Feature>),
    cancellable: Cancellable,
    on_failure: Option<Box<dyn Fn()>>,
    is_snap_history: bool,
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
                Ok((response, connection)) => match response {
                    // https://geminiprotocol.net/docs/protocol-specification.gmi#input-expected
                    Response::Input(input) => {
                        let title = input.to_string();
                        page.set_progress(0.0);
                        page.set_title(&title);
                        if is_snap_history {
                            page.snap_history();
                        }
                        redirects.replace(0); // reset
                        match input {
                            // https://geminiprotocol.net/docs/protocol-specification.gmi#status-10
                            Input::Default { message } => page.input.set_new_response(
                                page.item_action.clone(),
                                uri,
                                Some(message.as_ref().unwrap_or(&title)),
                                Some(1024),
                            ),
                            // https://geminiprotocol.net/docs/protocol-specification.gmi#status-11-sensitive-input
                            Input::Sensitive { message } => page.input.set_new_sensitive(
                                page.item_action.clone(),
                                uri,
                                Some(message.as_ref().unwrap_or(&title)),
                                Some(1024),
                            )
                        }
                    }
                    // https://geminiprotocol.net/docs/protocol-specification.gmi#status-20
                    Response::Success(success) => match *feature {
                        Feature::Download => {
                            // Init download widget
                            let status = page.content.to_status_download(
                                crate::tool::uri_to_title(&uri).trim_matches(MAIN_SEPARATOR), // grab default filename from base URI,
                                // format FS entities
                                &cancellable,
                                {
                                    let cancellable = cancellable.clone();
                                    let stream = connection.stream();
                                    move |file, action| match file.replace(
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
                                            file_output_stream::from_stream_async(
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
                                },
                            );
                            page.set_progress(0.0);
                            page.set_title(&status.title());
                            if is_snap_history {
                                page.snap_history();
                            }
                            redirects.replace(0); // reset
                        },
                        _ => match success.mime() {
                            "text/gemini" | "text/plain" => memory_input_stream::from_stream_async(
                                connection.stream(),
                                Priority::DEFAULT,
                                cancellable.clone(),
                                (
                                    0x400,   // 1024 chunk
                                    0xfffff, // 1M limit
                                ),
                                (
                                    |_, _| {},                   // on chunk (maybe nothing to count yet @TODO)
                                    move |result| match result { // on complete
                                        Ok((memory_input_stream, total)) => memory_input_stream.read_all_async(
                                            vec![0;total],
                                            Priority::DEFAULT,
                                            Some(&cancellable),
                                            move |result| match result {
                                                Ok((buffer, _ ,_)) => match std::str::from_utf8(&buffer) {
                                                    Ok(data) => {
                                                        let widget = if matches!(*feature, Feature::Source) {
                                                            page.content.to_text_source(data)
                                                        } else {
                                                            match success.mime() {
                                                                "text/gemini" => page.content.to_text_gemini(&uri, data),
                                                                "text/plain" => page.content.to_text_plain(data),
                                                                _ => panic!() // unexpected
                                                            }
                                                        };
                                                        page.search.set(Some(widget.text_view));
                                                        page.set_title(&match widget.meta.title {
                                                            Some(title) => title.into(), // @TODO
                                                            None => crate::tool::uri_to_title(&uri),
                                                        });
                                                        page.set_progress(0.0);
                                                        page.window_action
                                                            .find
                                                            .simple_action
                                                            .set_enabled(true);
                                                        if is_snap_history {
                                                            page.snap_history();
                                                        }
                                                        redirects.replace(0); // reset
                                                    },
                                                    Err(e) => {
                                                        let status = page.content.to_status_failure();
                                                        status.set_description(Some(&e.to_string()));
                                                        page.set_progress(0.0);
                                                        page.set_title(&status.title());
                                                        if is_snap_history {
                                                            page.snap_history();
                                                        }
                                                        redirects.replace(0); // reset
                                                    },
                                                },
                                                Err((_, e)) => {
                                                    let status = page.content.to_status_failure();
                                                    status.set_description(Some(&e.to_string()));
                                                    page.set_progress(0.0);
                                                    page.set_title(&status.title());
                                                    if is_snap_history {
                                                        page.snap_history();
                                                    }
                                                    redirects.replace(0); // reset
                                                }
                                            }
                                        ),
                                        Err(e) => {
                                            let status = page.content.to_status_failure();
                                            status.set_description(Some(&e.to_string()));
                                            page.set_progress(0.0);
                                            page.set_title(&status.title());
                                            if is_snap_history {
                                                page.snap_history();
                                            }
                                            redirects.replace(0); // reset
                                        },
                                    }
                                )
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
                                memory_input_stream::from_stream_async(
                                    connection.stream(),
                                    Priority::DEFAULT,
                                    cancellable.clone(),
                                    (
                                        0x400,   // 1024 bytes per chunk, optional step for images download tracking
                                        0xA00000 // 10M bytes max to prevent memory overflow if server play with promises
                                    ),
                                    (
                                        move |_, total| status.set_description(Some(&format!("Download: {total} bytes"))),
                                        {
                                            move | result | match result {
                                                Ok((memory_input_stream, _)) => {
                                                    Pixbuf::from_stream_async(
                                                        &memory_input_stream,
                                                        Some(&cancellable),
                                                        move |result| {
                                                            match result {
                                                                Ok(buffer) => {
                                                                    page.set_title(&crate::tool::uri_to_title(&uri));
                                                                    page.content.to_image(&Texture::for_pixbuf(&buffer));
                                                                }
                                                                Err(e) => {
                                                                    let status = page.content.to_status_failure();
                                                                    status.set_description(Some(e.message()));
                                                                    page.set_title(&status.title());
                                                                }
                                                            }
                                                            page.set_progress(0.0);
                                                            if is_snap_history {
                                                                page.snap_history();
                                                            }
                                                            redirects.replace(0); // reset
                                                        },
                                                    )
                                                }
                                                Err(e) => {
                                                    let status = page.content.to_status_failure();
                                                    status.set_description(Some(&e.to_string()));
                                                    page.set_progress(0.0);
                                                    page.set_title(&status.title());
                                                    if is_snap_history {
                                                        page.snap_history();
                                                    }
                                                    redirects.replace(0); // reset
                                                }
                                            }
                                        }
                                    ),
                                )
                            }
                            mime => {
                                let status = page
                                    .content
                                    .to_status_mime(mime, Some((&page.item_action, &uri)));
                                status.set_description(Some(&format!("Content type `{mime}` yet not supported")));
                                page.set_progress(0.0);
                                page.set_title(&status.title());
                                if is_snap_history {
                                    page.snap_history();
                                }
                                redirects.replace(0); // reset
                            },
                        }
                    },
                    // https://geminiprotocol.net/docs/protocol-specification.gmi#status-30-temporary-redirection
                    // https://geminiprotocol.net/docs/protocol-specification.gmi#status-31-permanent-redirection
                    Response::Redirect(redirect) => match redirect.to_uri(&uri) {
                        Ok(target) => {
                            // Increase client redirection counter
                            let total = redirects.take() + 1;
                            // > Client MUST limit the number of redirections they follow to 5 redirections
                            // > https://geminiprotocol.net/docs/protocol-specification.gmi#redirection
                            if total > 5 {
                                let status = page.content.to_status_failure();
                                status.set_description(Some("Redirection limit reached"));
                                page.set_progress(0.0);
                                page.set_title(&status.title());
                                redirects.replace(0); // reset
                            /* @TODO can't find that in specification, implement manual confirmation status page:
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
                                if matches!(redirect, Redirect::Permanent { .. }) {
                                    page.navigation.set_request(&target.to_string());
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
                    Response::Certificate(ref certificate) => match certificate {
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-60
                        Certificate::Required { message } |
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-61-certificate-not-authorized
                        Certificate::NotAuthorized { message } |
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-62-certificate-not-valid
                        Certificate::NotValid { message } => {
                            let status = page.content.to_status_identity();
                            status.set_description(Some(message.as_ref().unwrap_or(&certificate.to_string())));
                            page.set_progress(0.0);
                            page.set_title(&status.title());
                            if is_snap_history {
                                page.snap_history();
                            }
                            redirects.replace(0); // reset
                        }
                    }
                    Response::Failure(failure) => match failure {
                        Failure::Temporary(ref temporary) => match temporary {
                            Temporary::CgiError { message } |
                            Temporary::Default { message } |
                            Temporary::ProxyError { message } |
                            Temporary::ServerUnavailable { message } |
                            Temporary::SlowDown { message } => {
                                let status = page.content.to_status_failure();
                                status.set_description(Some(message.as_ref().unwrap_or(&temporary.to_string())));
                                page.set_progress(0.0);
                                page.set_title(&status.title());
                                if is_snap_history {
                                    page.snap_history();
                                }
                                redirects.replace(0); // reset
                                if let Some(callback) = on_failure {
                                    callback()
                                }
                            }
                        }
                        Failure::Permanent(ref permanent) => match permanent {
                            Permanent::BadRequest { message } |
                            Permanent::Default { message } |
                            Permanent::Gone { message } |
                            Permanent::NotFound { message } |
                            Permanent::ProxyRequestRefused { message } => {
                                let status = page.content.to_status_failure();
                                status.set_description(Some(message.as_ref().unwrap_or(&permanent.to_string())));
                                page.set_progress(0.0);
                                page.set_title(&status.title());
                                if is_snap_history {
                                    page.snap_history();
                                }
                                redirects.replace(0); // reset
                                if let Some(callback) = on_failure {
                                    callback()
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    let status = page.content.to_status_failure();
                    status.set_description(Some(&e.to_string()));
                    page.set_progress(0.0);
                    page.set_title(&status.title());
                    if is_snap_history {
                        page.snap_history();
                    }
                    redirects.replace(0); // reset
                }
            }
        },
    )
}
