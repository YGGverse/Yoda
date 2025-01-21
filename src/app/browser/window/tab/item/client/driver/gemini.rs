use super::{Feature, Page};
use ggemini::client::{
    connection::response::{data::Text, meta::Status},
    Request,
};
use gtk::glib::{GString, UriFlags};
use gtk::prelude::{EditableExt, FileExt};
use gtk::{
    gdk::Texture,
    gdk_pixbuf::Pixbuf,
    gio::{Cancellable, SocketClientEvent},
    glib::{Priority, Uri},
    prelude::{EntryExt, SocketClientExt},
};
use std::{cell::Cell, path::MAIN_SEPARATOR, rc::Rc, time::Duration};

/// Multi-protocol client API for `Page` object
pub struct Gemini {
    /// Should be initiated once
    client: Rc<ggemini::Client>,
    /// Handle target
    page: Rc<Page>,
    /// Validate redirection count by Gemini protocol specification
    redirects: Rc<Cell<usize>>,
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
                page.navigation
                    .request
                    .widget
                    .entry
                    .set_progress_fraction(match event {
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
            page: page.clone(),
            redirects: Rc::new(Cell::new(0)),
        }
    }

    // Actions

    pub fn handle(&self, uri: Uri, feature: Feature, cancellable: Cancellable, is_history: bool) {
        // Move focus out from navigation entry
        self.page
            .browser_action
            .escape
            .activate_stateful_once(Some(self.page.id.as_str().into()));

        // Initially disable find action
        self.page
            .window_action
            .find
            .simple_action
            .set_enabled(false);

        // Reset widgets
        self.page.search.unset();
        self.page.input.unset();
        self.page.title.replace("Loading..".into());

        // Begin action
        self.page
            .navigation
            .request
            .widget
            .entry
            .set_progress_fraction(0.1);

        self.page
            .browser_action
            .update
            .activate(Some(&self.page.id));

        if is_history {
            snap_history(&self.page, None);
        }

        self.client.request_async(
            Request::gemini(uri.clone()),
            Priority::DEFAULT,
            cancellable.clone(),
            // Search for user certificate match request
            // * @TODO this feature does not support multi-protocol yet
            match self
                .page
                .profile
                .identity
                .gemini
                .match_scope(&uri.to_string())
            {
                Some(identity) => match identity.to_tls_certificate() {
                    Ok(certificate) => Some(certificate),
                    Err(_) => panic!(), // unexpected
                },
                None => None,
            },
            {
                let uri = uri.clone();
                let page = self.page.clone();
                let redirects = self.redirects.clone();
                move |result| match result {
                    Ok(response) => {
                        match response.meta.status {
                            // https://geminiprotocol.net/docs/protocol-specification.gmi#input-expected
                            Status::Input => {
                                let title = match response.meta.data {
                                    Some(data) => data.to_string(),
                                    None => Status::Input.to_string(),
                                };
                                page.input.set_new_response(
                                    page.tab_action.clone(),
                                    uri,
                                    Some(&title),
                                    Some(1024),
                                );
                                page.title.replace(title.into()); // @TODO
                                page.browser_action.update.activate(Some(&page.id));
                            }
                            Status::SensitiveInput => {
                                let title = match response.meta.data {
                                    Some(data) => data.to_string(),
                                    None => Status::Input.to_string(),
                                };
                                page.input.set_new_sensitive(
                                    page.tab_action.clone(),
                                    uri,
                                    Some(&title),
                                    Some(1024),
                                );
                                page.title.replace(title.into()); // @TODO
                                page.browser_action.update.activate(Some(&page.id));
                            }
                            // https://geminiprotocol.net/docs/protocol-specification.gmi#status-20
                            Status::Success => match feature {
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
                                                                    move |_, total| {
                                                                        action.update.activate(&format!(
                                                                            "Received {}...",
                                                                            crate::tool::format_bytes(total)
                                                                        ))
                                                                    }
                                                                },
                                                                // on complete
                                                                {
                                                                    let action = action.clone();
                                                                    move |result| match result {
                                                                        Ok((_, total)) => {
                                                                            action.complete.activate(&format!(
                                                                                "Saved to {} ({total} bytes total)",
                                                                                file.parse_name()
                                                                            ))
                                                                        }
                                                                        Err(e) => {
                                                                            action.cancel.activate(&e.to_string())
                                                                        }
                                                                    }
                                                                },
                                                            ),
                                                        );
                                                    }
                                                    Err(e) => action.cancel.activate(&e.to_string()),
                                                }
                                            }
                                        },
                                    );
                                    page.title.replace(status.title());
                                    page.browser_action.update.activate(Some(&page.id));
                                },
                                _ => match response.meta.mime {
                                    Some(mime) => match mime.as_str() {
                                        "text/gemini" => Text::from_stream_async(
                                            response.connection.stream(),
                                            Priority::DEFAULT,
                                            cancellable.clone(),
                                            move |result| match result {
                                                Ok(text) => {
                                                    /* @TODO refactor features
                                                    let widget = if is_source_request {
                                                        page.content.to_text_source(&data)
                                                    } else {
                                                        page.content.to_text_gemini(&uri, &data)
                                                    };*/

                                                    let widget = page
                                                        .content
                                                        .to_text_gemini(&uri, &text.to_string());

                                                    // Connect `TextView` widget, update `search` model
                                                    page.search.set(Some(widget.text_view));

                                                    // Update page meta
                                                    page.title.replace(match widget.meta.title {
                                                        Some(title) => title.into(), // @TODO
                                                        None => uri_to_title(&uri),
                                                    });

                                                    // Deactivate progress fraction
                                                    page.navigation.request.widget.entry.set_progress_fraction(0.0);

                                                    // Update window components
                                                    page.window_action
                                                        .find
                                                        .simple_action
                                                        .set_enabled(true);

                                                    page.browser_action.update.activate(Some(&page.id));
                                                }
                                                Err(e) => {
                                                    let status = page.content.to_status_failure();
                                                    status.set_description(Some(&e.to_string()));

                                                    page.title.replace(status.title());
                                                    page.browser_action.update.activate(Some(&page.id));
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
                                                move |_, total| {
                                                    // Update loading progress
                                                    status.set_description(Some(&format!("Download: {total} bytes")));
                                                },
                                                {
                                                    let page = page.clone();
                                                    move |result| match result {
                                                        Ok((memory_input_stream, _)) => {
                                                            Pixbuf::from_stream_async(
                                                                &memory_input_stream,
                                                                Some(&cancellable),
                                                                move |result| {
                                                                    // Process buffer data
                                                                    match result {
                                                                        Ok(buffer) => {
                                                                            page.title.replace(uri_to_title(&uri));
                                                                            page.content
                                                                                .to_image(&Texture::for_pixbuf(&buffer));
                                                                            page.browser_action
                                                                                .update
                                                                                .activate(Some(&page.id));
                                                                        }
                                                                        Err(e) => {
                                                                            let status = page.content.to_status_failure();
                                                                            status.set_description(Some(e.message()));
                                                                            page.title.replace(status.title());
                                                                        }
                                                                    };
                                                                    page.browser_action.update.activate(Some(&page.id));
                                                                },
                                                            )
                                                        }
                                                        Err(e) => {
                                                            let status = page.content.to_status_failure();
                                                            status.set_description(Some(&e.to_string()));

                                                            page.title.replace(status.title());
                                                            page.browser_action.update.activate(Some(&page.id));
                                                        }
                                                    }
                                                },
                                            );
                                        }
                                        mime => {
                                            let status = page
                                                .content
                                                .to_status_mime(mime, Some((&page.tab_action, &uri)));
                                            status.set_description(Some(&format!("Content type `{mime}` yet not supported")));

                                            page.title.replace(status.title());
                                            page.browser_action.update.activate(Some(&page.id));
                                        },
                                    },
                                    None => {
                                        let status = page.content.to_status_failure();
                                        status.set_description(Some("MIME type not found"));

                                        page.title.replace(status.title());
                                        page.browser_action.update.activate(Some(&page.id));
                                    },
                                }
                            },
                            // https://geminiprotocol.net/docs/protocol-specification.gmi#status-30-temporary-redirection
                            // https://geminiprotocol.net/docs/protocol-specification.gmi#status-31-permanent-redirection
                            Status::PermanentRedirect | Status::Redirect => {
                                // Expected target URL in response meta
                                match response.meta.data {
                                    Some(data) => {
                                        match uri.parse_relative(data.as_str(), UriFlags::NONE) {
                                            Ok(target) => {
                                                let total = redirects.take() + 1;

                                                // Validate total redirects by protocol specification
                                                if total > 5 {
                                                    let status = page.content.to_status_failure();
                                                    status.set_description(Some("Redirection limit reached"));

                                                    page.title.replace(status.title());
                                                    page.browser_action.update.activate(Some(&page.id));

                                                    redirects.replace(0); // reset

                                                // Disallow external redirection
                                                } else if uri.scheme() != target.scheme()
                                                    || uri.port() != target.port()
                                                    || uri.host() != target.host() {
                                                        let status = page.content.to_status_failure();
                                                        status.set_description(Some("External redirects not allowed by protocol specification"));

                                                        page.title.replace(status.title());
                                                        page.browser_action.update.activate(Some(&page.id));

                                                        redirects.replace(0); // reset
                                                // Valid
                                                } else {
                                                    if matches!(response.meta.status, Status::PermanentRedirect) {
                                                        page.navigation
                                                        .request
                                                        .widget
                                                        .entry
                                                        .set_text(&uri.to_string());
                                                    }
                                                    redirects.replace(total);
                                                    page.tab_action.load.activate(Some(&target.to_string()), is_history);
                                                }
                                            }
                                            Err(e) => {
                                                let status = page.content.to_status_failure();
                                                status.set_description(Some(&e.to_string()));

                                                page.title.replace(status.title());
                                                page.browser_action.update.activate(Some(&page.id));
                                            }
                                        }
                                    }
                                    None => {
                                        let status = page.content.to_status_failure();
                                        status.set_description(Some("Redirection target not found"));

                                        page.title.replace(status.title());
                                        page.browser_action.update.activate(Some(&page.id));
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

                                page.title.replace(status.title());
                                page.browser_action.update.activate(Some(&page.id));
                            }
                            status => {
                                let _status = page.content.to_status_failure();
                                _status.set_description(Some(&format!("Undefined status code `{status}`")));

                                page.title.replace(_status.title());
                                page.browser_action.update.activate(Some(&page.id));
                            },
                        }
                    }
                    Err(e) => {
                        let status = page.content.to_status_failure();
                        status.set_description(Some(&e.to_string()));

                        page.title.replace(status.title());
                        page.browser_action.update.activate(Some(&page.id));
                    },
                }
            },
        )
    }
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

/// Make new history record in related components
/// * optional [Uri](https://docs.gtk.org/glib/struct.Uri.html) reference wanted only for performance reasons, to not parse it twice
fn snap_history(page: &Page, uri: Option<&Uri>) {
    let request = page.navigation.request.widget.entry.text();

    // Add new record into the global memory index (used in global menu)
    // * if the `Uri` is `None`, try parse it from `request`
    match uri {
        Some(uri) => page.profile.history.memory.request.set(uri.clone()),
        None => {
            // this case especially useful for some routes that contain redirects
            // maybe some parental optimization wanted @TODO
            if let Some(uri) = page.navigation.request.as_uri() {
                page.profile.history.memory.request.set(uri);
            }
        }
    }

    // Add new record into the page navigation history
    if match page.navigation.history.current() {
        Some(current) => current != request, // apply additional filters
        None => true,
    } {
        page.navigation.history.add(request, true)
    }
}
