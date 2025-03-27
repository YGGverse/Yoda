use super::{Feature, Page};
use ggemini::client::connection::response::{Input, Redirect};
use ggemini::{
    client::{Client, Request, Response},
    gio::{file_output_stream, memory_input_stream},
};
use gtk::{
    gdk::Texture,
    gdk_pixbuf::Pixbuf,
    gio::{Cancellable, SocketClientEvent},
    glib::{Priority, Uri},
    prelude::{ButtonExt, FileExt, SocketClientExt},
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
            let p = page.clone();
            move |_, event, _, _| {
                let mut i = p.navigation.request.info.borrow_mut();
                p.set_progress(match event {
                    // 0.1 reserved for handle begin
                    SocketClientEvent::Resolving => {
                        i.reset().add_event("Resolving".to_string());
                        0.2
                    }
                    SocketClientEvent::Resolved => {
                        i.add_event("Resolved".to_string());
                        0.3
                    }
                    SocketClientEvent::Connecting => {
                        i.add_event("Connecting".to_string());
                        0.4
                    }
                    SocketClientEvent::Connected => {
                        i.add_event("Connected".to_string());
                        0.5
                    }
                    SocketClientEvent::ProxyNegotiating => {
                        i.add_event("Proxy negotiating".to_string());
                        0.6
                    }
                    SocketClientEvent::ProxyNegotiated => {
                        i.add_event("Proxy negotiated".to_string());
                        0.7
                    }
                    // * `TlsHandshaking` | `TlsHandshaked` has effect only for guest connections!
                    SocketClientEvent::TlsHandshaking => {
                        i.add_event("TLS handshaking".to_string());
                        0.8
                    }
                    SocketClientEvent::TlsHandshaked => {
                        i.add_event("TLS handshaked".to_string());
                        0.9
                    }
                    SocketClientEvent::Complete => {
                        i.add_event("Receiving".to_string());
                        1.0
                    }
                    _ => panic!(), // alert on API change
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
        use ggemini::client::connection::request::{Mode, Request};
        match uri.scheme().as_str() {
            "gemini" => handle(
                self,
                Request::Gemini {
                    uri,
                    mode: Mode::HeaderOnly,
                },
                feature,
                cancellable,
                None,
                is_snap_history,
            ),
            "titan" => {
                self.page.input.set_new_titan({
                    let this = Self {
                        client: self.client.clone(),
                        page: self.page.clone(),
                        redirects: self.redirects.clone(),
                    };
                    move |header, bytes, on_failure| {
                        handle(
                            &this,
                            Request::Titan {
                                uri: uri.clone(),
                                data: bytes,
                                mime: header.mime.map(|mime| mime.into()),
                                token: header.token.map(|token| token.into()),
                                mode: Mode::HeaderOnly,
                            },
                            feature.clone(),
                            cancellable.clone(),
                            Some(on_failure),
                            is_snap_history,
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
    this: &Gemini,
    request: Request,
    feature: Rc<Feature>,
    cancellable: Cancellable,
    on_failure: Option<Box<dyn Fn()>>,
    is_snap_history: bool,
) {
    const EVENT_COMPLETED: &str = "Completed";
    let uri = request.uri().clone();
    this.client.request_async(
        request,
        Priority::DEFAULT,
        cancellable.clone(),
        // Search for user certificate match request
        // * @TODO this feature does not support multi-protocol yet
        this.page
            .profile
            .identity
            .get(&uri.to_string()).map(|identity|identity.to_tls_certificate().unwrap()),
        {
            let page = this.page.clone();
            let redirects = this.redirects.clone();
            move |result| match result {
                Ok((response, connection)) => {
                    /// Common page info pattern for some cases in the current scope
                    /// * includes commit action!
                    fn update_page_info(page: &Page, event_name: &str) {
                        let mut i = page.navigation.request.info.borrow_mut();
                        i.add_event(event_name.to_string());
                        page.navigation.request.update_secondary_icon(&i)
                    }
                    // Update socket info at the point, where the connection is active yet
                    // * also, actualize `request` as same everywhere below
                    {
                        use gtk::prelude::SocketConnectionExt;
                        let mut i = page.navigation.request.info.borrow_mut();
                        i
                            .set_request(Some(uri.to_string()))
                            .set_socket(Some((
                                connection.socket_connection.local_address().unwrap(),
                                connection.socket_connection.remote_address().unwrap()
                            )));
                            // * unwrap fails only on `connection.socket_connection.is_closed()`
                            //   drop the panic as unexpected here.
                    }
                    // Handle response
                    match response {
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#input-expected
                        Response::Input(input) => {
                            page.set_progress(0.0);
                            page.set_title("Input expected");
                            if is_snap_history {
                                page.snap_history();
                            }
                            redirects.replace(0); // reset
                            let mut i = page.navigation.request.info.borrow_mut();
                            i
                                .add_event(EVENT_COMPLETED.to_string())
                                .set_header(Some(input.as_str().to_string()))
                                .set_size(Some(input.as_bytes().len()));
                            page.navigation.request.update_secondary_icon(&i);
                            match input {
                                // https://geminiprotocol.net/docs/protocol-specification.gmi#status-10
                                Input::Default(d) => page.input.set_new_response(
                                    page.item_action.clone(),
                                    uri,
                                    Some(d.message_or_default()),
                                    Some(1024),
                                ),
                                // https://geminiprotocol.net/docs/protocol-specification.gmi#status-11-sensitive-input
                                Input::Sensitive(s) => page.input.set_new_sensitive(
                                    page.item_action.clone(),
                                    uri,
                                    Some(s.message_or_default()),
                                    Some(1024),
                                )
                            }
                        }
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-20
                        Response::Success(success) => match *feature {
                            Feature::Download => {
                                // Init download widget
                                let s = page.content.to_status_download(
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
                                                    file_output_stream::Size {
                                                        chunk: 0x100000, // 1M bytes per chunk
                                                        limit: None,     // unlimited
                                                        total: 0,        // initial totals
                                                    },
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
                                page.set_title(&s.title());
                                if is_snap_history {
                                    page.snap_history();
                                }
                                redirects.replace(0); // reset
                            },
                            _ => match success.mime() {
                                Ok(mime) => match mime.as_str() {
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
                                                    vec![0; total],
                                                    Priority::DEFAULT,
                                                    Some(&cancellable),
                                                    {
                                                        let m = mime.clone();
                                                        move |result| match result {
                                                            Ok((buffer, _ ,_)) => match std::str::from_utf8(&buffer) {
                                                                Ok(data) => {
                                                                    let mut i = page.navigation.request.info.borrow_mut();
                                                                    i
                                                                        .add_event("Parsing".to_string())
                                                                        .set_header(Some(success.as_header_str().to_string()))
                                                                        .set_size(Some(buffer.len()))
                                                                        .set_mime(Some(mime));
                                                                    let w = if matches!(*feature, Feature::Source) {
                                                                        page.content.to_text_source(data)
                                                                    } else {
                                                                        match m.as_str() {
                                                                            "text/gemini" => page.content.to_text_gemini(&uri, data),
                                                                            "text/plain" => page.content.to_text_plain(data),
                                                                            _ => panic!() // unexpected
                                                                        }
                                                                    };
                                                                    i.add_event("Parsed".to_string());
                                                                    page.search.set(Some(w.text_view));
                                                                    page.set_title(&match w.meta.title {
                                                                        Some(t) => t.into(), // @TODO
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
                                                                    i.add_event(EVENT_COMPLETED.to_string());
                                                                    page.navigation.request.update_secondary_icon(&i)
                                                                },
                                                                Err(e) => {
                                                                    let s = page.content.to_status_failure();
                                                                    s.set_description(Some(&e.to_string()));
                                                                    page.set_progress(0.0);
                                                                    page.set_title(&s.title());
                                                                    if is_snap_history {
                                                                        page.snap_history();
                                                                    }
                                                                    redirects.replace(0); // reset
                                                                    update_page_info(&page, EVENT_COMPLETED);
                                                                },
                                                            },
                                                            Err((_, e)) => {
                                                                let s = page.content.to_status_failure();
                                                                s.set_description(Some(&e.to_string()));
                                                                page.set_progress(0.0);
                                                                page.set_title(&s.title());
                                                                if is_snap_history {
                                                                    page.snap_history();
                                                                }
                                                                redirects.replace(0); // reset
                                                                update_page_info(&page, EVENT_COMPLETED);
                                                            }
                                                        }
                                                    }
                                                ),
                                                Err(e) => {
                                                    let s = page.content.to_status_failure();
                                                    s.set_description(Some(&e.to_string()));
                                                    page.set_progress(0.0);
                                                    page.set_title(&s.title());
                                                    if is_snap_history {
                                                        page.snap_history();
                                                    }
                                                    redirects.replace(0); // reset
                                                    update_page_info(&page, EVENT_COMPLETED);
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
                                                                            let mut i = page.navigation.request.info.borrow_mut();
                                                                            i
                                                                                .add_event(EVENT_COMPLETED.to_string())
                                                                                .set_header(Some(success.as_header_str().to_string()))
                                                                                .set_mime(Some(mime))
                                                                                .set_size(Some(buffer.byte_length()));
                                                                            page.navigation.request.update_secondary_icon(&i)
                                                                        }
                                                                        Err(e) => {
                                                                            let s = page.content.to_status_failure();
                                                                            s.set_description(Some(e.message()));
                                                                            page.set_title(&s.title());
                                                                            update_page_info(&page, EVENT_COMPLETED);
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
                                                            let s = page.content.to_status_failure();
                                                            s.set_description(Some(&e.to_string()));
                                                            page.set_progress(0.0);
                                                            page.set_title(&s.title());
                                                            if is_snap_history {
                                                                page.snap_history();
                                                            }
                                                            redirects.replace(0); // reset
                                                            update_page_info(&page, EVENT_COMPLETED);
                                                        }
                                                    }
                                                }
                                            ),
                                        )
                                    }
                                    mime => {
                                        let s = page
                                            .content
                                            .to_status_mime(mime, Some((&page.item_action, &uri)));
                                        s.set_description(Some(&format!("Content type `{mime}` yet not supported")));
                                        page.set_progress(0.0);
                                        page.set_title(&s.title());
                                        if is_snap_history {
                                            page.snap_history();
                                        }
                                        redirects.replace(0); // reset
                                        let mut i = page.navigation.request.info.borrow_mut();
                                        i
                                            .add_event(EVENT_COMPLETED.to_string())
                                            .set_header(Some(success.as_header_str().to_string()))
                                            .set_mime(Some(mime.to_string()));
                                        page.navigation.request.update_secondary_icon(&i)
                                    },
                                },
                                Err(e) => {
                                    let s = page.content.to_status_failure();
                                    s.set_description(Some(&e.to_string()));
                                    page.set_progress(0.0);
                                    page.set_title(&s.title());
                                    if is_snap_history {
                                        page.snap_history();
                                    }
                                    redirects.replace(0); // reset
                                    update_page_info(&page, EVENT_COMPLETED);
                                }
                            }
                        },
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-30-temporary-redirection
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-31-permanent-redirection
                        Response::Redirect(redirect) => match redirect.uri(&uri) {
                            Ok(target) => {
                                // Increase client redirection counter
                                let total = redirects.take() + 1;
                                // > Client MUST limit the number of redirections they follow to 5 redirections
                                // > https://geminiprotocol.net/docs/protocol-specification.gmi#redirection
                                if total > 5 {
                                    let s = page.content.to_status_failure();
                                    s.set_description(Some("Redirection limit reached"));
                                    page.set_progress(0.0);
                                    page.set_title(&s.title());
                                    redirects.replace(0); // reset
                                    update_page_info(&page, EVENT_COMPLETED);
                                // Disallow external redirection by default as potentially unsafe
                                // even not specified, require follow confirmation @TODO optional
                                } else if uri.host() != target.host() {
                                    let u = target.to_string();
                                    let s = page.content.to_status_failure();
                                    let t = "External redirection";
                                    s.set_title(t);
                                    s.set_icon_name(Some("dialog-warning-symbolic"));
                                    s.set_description(Some(&u));
                                    s.set_child(Some(&{
                                        let b = gtk::Button::builder()
                                            .css_classes(["suggested-action"])
                                            .halign(gtk::Align::Center)
                                            .label("Follow")
                                            .build();
                                        b.connect_clicked({
                                            let p = page.clone();
                                            move |_| p.item_action.load.activate(Some(&u), false)
                                        });
                                        b
                                    }));
                                    page.set_progress(0.0);
                                    page.set_title(t);
                                    redirects.replace(0); // reset
                                    update_page_info(&page, EVENT_COMPLETED);
                                } else {
                                    let t = target.to_string();
                                    if matches!(redirect, Redirect::Permanent { .. }) {
                                        page.navigation.set_request(&t);
                                    }
                                    redirects.replace(total);
                                    {
                                        let mut i = page.navigation.request.info.take();
                                        i
                                            .add_event(EVENT_COMPLETED.to_string())
                                            .set_header(Some(redirect.as_str().to_string()));
                                        page.navigation.request.info.replace(match redirect {
                                            Redirect::Permanent { .. } => i.into_permanent_redirect(),
                                            Redirect::Temporary { .. } => i.into_temporary_redirect(),
                                        });
                                    }
                                    page.item_action.load.activate(Some(&t), false);
                                }
                            }
                            Err(e) => {
                                let s = page.content.to_status_failure();
                                s.set_description(Some(&e.to_string()));
                                page.set_progress(0.0);
                                page.set_title(&s.title());
                                redirects.replace(0); // reset
                                update_page_info(&page, EVENT_COMPLETED);
                            }
                        }
                        Response::Certificate(certificate) => {
                            // update page information
                            let mut i = page.navigation.request.info.borrow_mut();
                            i
                                .add_event(EVENT_COMPLETED.to_string())
                                .set_header(Some(certificate.as_str().to_string()))
                                .set_size(Some(certificate.as_bytes().len()));
                            page.navigation.request.update_secondary_icon(&i);
                            // update page content widget
                            let s = page.content.to_status_identity();
                            s.set_description(Some(certificate.message_or_default()));
                            // update other page members
                            page.set_progress(0.0);
                            page.set_title(certificate.message_or_default());
                            if is_snap_history {
                                page.snap_history();
                            }
                            // reset previous redirections
                            redirects.replace(0);
                        }
                        Response::Failure(failure) => {
                            let s = page.content.to_status_failure();
                            s.set_description(Some(failure.message_or_default()));
                            page.set_progress(0.0);
                            page.set_title(&s.title());
                            if is_snap_history {
                                page.snap_history();
                            }
                            redirects.replace(0); // reset
                            update_page_info(&page, EVENT_COMPLETED);
                            if let Some(callback) = on_failure {
                                callback()
                            }
                        }
                    }
                }
                Err(e) => {
                    let s = page.content.to_status_failure();
                    s.set_description(Some(&e.to_string()));
                    page.set_progress(0.0);
                    page.set_title(&s.title());
                    if is_snap_history {
                        page.snap_history();
                    }
                    redirects.replace(0); // reset
                    let mut i = page.navigation.request.info.borrow_mut();
                    i.add_event(EVENT_COMPLETED.to_string()).set_request(Some(uri.to_string()));
                    page.navigation.request.update_secondary_icon(&i)
                }
            }
        },
    )
}
