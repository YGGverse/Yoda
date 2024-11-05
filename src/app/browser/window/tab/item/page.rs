mod content;
mod database;
mod input;
mod meta;
mod navigation;
mod widget;

use content::Content;
use database::Database;
use input::Input;
use navigation::Navigation;
use widget::Widget;

use meta::{Meta, Status};

use gtk::{
    gdk_pixbuf::Pixbuf,
    gio::{
        Cancellable, SimpleAction, SocketClient, SocketClientEvent, SocketProtocol,
        TlsCertificateFlags,
    },
    glib::{
        gformat, uuid_string_random, Bytes, GString, Priority, Regex, RegexCompileFlags,
        RegexMatchFlags, Uri, UriFlags, UriHideFlags,
    },
    prelude::{
        ActionExt, IOStreamExt, OutputStreamExt, SocketClientExt, StaticVariantType, ToVariant,
    },
    Box,
};
use sqlite::Transaction;
use std::{sync::Arc, time::Duration};

pub struct Page {
    id: GString,
    // Actions
    action_page_open: SimpleAction,
    action_page_reload: SimpleAction,
    action_update: SimpleAction,
    // Components
    navigation: Arc<Navigation>,
    content: Arc<Content>,
    input: Arc<Input>,
    // Extras
    meta: Arc<Meta>,
    // GTK
    widget: Arc<Widget>,
}

impl Page {
    // Construct
    pub fn new_arc(
        id: GString,
        action_tab_open: SimpleAction,
        action_page_home: SimpleAction,
        action_page_history_back: SimpleAction,
        action_page_history_forward: SimpleAction,
        action_page_reload: SimpleAction,
        action_update: SimpleAction,
    ) -> Arc<Self> {
        // Init local actions
        let action_page_open =
            SimpleAction::new(&uuid_string_random(), Some(&String::static_variant_type()));

        // Init components
        let content = Content::new_arc(action_tab_open.clone(), action_page_open.clone());

        let navigation = Navigation::new_arc(
            action_page_home.clone(),
            action_page_history_back.clone(),
            action_page_history_forward.clone(),
            action_page_reload.clone(),
            action_update.clone(),
        );

        let input = Input::new_arc();

        let widget = Widget::new_arc(
            &id,
            action_page_open.clone(),
            navigation.gobject(),
            content.gobject(),
            input.gobject(),
        );

        let meta = Meta::new_arc(Status::New, gformat!("New page"));

        // Init events
        action_page_open.connect_activate({
            let navigation = navigation.clone();
            let action_page_reload = action_page_reload.clone();
            move |_, request| {
                // Update request
                navigation.set_request_text(
                    request
                        .expect("Parameter required for this action")
                        .get::<String>()
                        .expect("Parameter does not match `String`")
                        .as_str(),
                );

                // Reload page
                action_page_reload.activate(None);
            }
        });

        // Return activated `Self`
        Arc::new(Self {
            id,
            // Actions
            action_page_open,
            action_page_reload,
            action_update,
            // Components
            content,
            navigation,
            input,
            // Extras
            meta,
            // GTK
            widget,
        })
    }

    // Actions

    pub fn home(&self) {
        if let Some(url) = self.navigation.home_url() {
            // Update with history record
            self.action_page_open.activate(Some(&url.to_variant()));
        }
    }

    pub fn history_back(&self) {
        if let Some(request) = self.navigation.history_back(true) {
            // Update
            self.navigation.set_request_text(&request);

            // Reload page
            self.action_page_reload.activate(None);
        }
    }

    pub fn history_forward(&self) {
        if let Some(request) = self.navigation.history_forward(true) {
            // Update
            self.navigation.set_request_text(&request);

            // Reload page
            self.action_page_reload.activate(None);
        }
    }

    pub fn reload(&self) {
        /// Global limit to prevent infinitive redirects (ALL protocols)
        /// * every protocol implementation has own value checker, according to specification
        const DEFAULT_MAX_REDIRECT_COUNT: i8 = 10;

        // Reset widgets
        self.input.unset();

        // Create shared variant value
        let id = self.id.to_variant();

        // Try **take** request value from Redirect holder first
        let request = if let Some(redirect) = self.meta.take_redirect() {
            // Update redirect counter
            self.meta
                .set_redirect_count(match self.meta.redirect_count() {
                    Some(value) => {
                        // Prevent infinitive redirection
                        if value > DEFAULT_MAX_REDIRECT_COUNT {
                            todo!()
                        }
                        // Increase
                        Some(value + 1)
                    }
                    // Set initial value
                    None => Some(1),
                });

            // Update navigation on redirect `is_foreground`
            if redirect.is_foreground() {
                self.navigation
                    .set_request_text(redirect.request().as_str());
            }

            // Return value from redirection holder
            redirect.request()
        } else {
            // Add history record
            let value = self.navigation.request_text();

            match self.navigation.history_current() {
                Some(current) => {
                    if current != value {
                        self.navigation.history_add(value);
                    }
                }
                None => self.navigation.history_add(value),
            }

            // Reset redirect counter as request value taken from user input
            self.meta.unset_redirect_count();

            // Return value from navigation entry
            self.navigation.request_text()
        };

        // Update
        self.meta.set_status(Status::Reload).set_title(&"Loading..");
        self.action_update.activate(Some(&id));

        // Route by request
        match Uri::parse(&request, UriFlags::NONE) {
            Ok(uri) => {
                // Route by scheme
                match uri.scheme().as_str() {
                    "file" => todo!(),
                    "gemini" => self.load_gemini(uri), // @TODO
                    scheme => {
                        // Define common data
                        let status = Status::Failure;
                        let title = &"Oops";

                        // Update widget
                        self.content
                            .to_status_failure()
                            .set_title(title)
                            .set_description(Some(
                                gformat!("Protocol `{scheme}` not supported").as_str(),
                            ));

                        // Update meta
                        self.meta.set_status(status).set_title(title);

                        // Update window
                        self.action_update.activate(Some(&id));
                    }
                }
            }
            Err(_) => {
                // Try interpret URI manually
                if Regex::match_simple(
                    r"^[^\/\s]+\.[\w]{2,}",
                    request.clone(),
                    RegexCompileFlags::DEFAULT,
                    RegexMatchFlags::DEFAULT,
                ) {
                    // Seems request contain some host, try append default scheme
                    let request = gformat!("gemini://{request}");
                    // Make sure new request conversable to valid URI
                    match Uri::parse(&request, UriFlags::NONE) {
                        Ok(_) => {
                            // Update
                            self.navigation.set_request_text(&request);

                            // Reload page
                            self.action_page_reload.activate(None);
                        }
                        Err(_) => {
                            // @TODO any action here?
                        }
                    }
                } else {
                    // Plain text given, make search request to default provider
                    let request = gformat!(
                        "gemini://tlgs.one/search?{}",
                        Uri::escape_string(&request, None, false)
                    );

                    // Update
                    self.navigation.set_request_text(&request);

                    // Reload page
                    self.action_page_reload.activate(None);
                }
            }
        }; // Uri::parse
    }

    pub fn update(&self) {
        // Update components
        self.navigation.update(self.progress_fraction());
        // @TODO self.content.update();
    }

    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_id: &i64,
    ) -> Result<(), String> {
        match Database::records(transaction, app_browser_window_tab_item_id) {
            Ok(records) => {
                for record in records {
                    match Database::delete(transaction, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to the item childs
                            self.meta.clean(transaction, &record.id)?;
                            self.navigation.clean(transaction, &record.id)?;
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn restore(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_id: &i64,
    ) -> Result<(), String> {
        // Update status
        self.meta.set_status(Status::SessionRestore);

        // Begin page restore
        match Database::records(transaction, app_browser_window_tab_item_id) {
            Ok(records) => {
                for record in records {
                    // Delegate restore action to the item childs
                    self.meta.restore(transaction, &record.id)?;
                    self.navigation.restore(transaction, &record.id)?;
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        // Update status
        self.meta.set_status(Status::SessionRestored);

        Ok(())
    }

    pub fn save(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_id: &i64,
    ) -> Result<(), String> {
        match Database::add(transaction, app_browser_window_tab_item_id) {
            Ok(_) => {
                let id = Database::last_insert_id(transaction);

                // Delegate save action to childs
                self.meta.save(transaction, &id)?;
                self.navigation.save(transaction, &id)?;
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn navigation_request_grab_focus(&self) {
        self.navigation.request_grab_focus();
    }

    // Setters

    pub fn set_navigation_request_text(&self, value: &str) {
        self.navigation.set_request_text(value);
    }

    // Getters

    pub fn progress_fraction(&self) -> Option<f64> {
        // Interpret status to progress fraction
        match self.meta.status() {
            Status::Reload | Status::SessionRestore => Some(0.0),
            Status::Resolving => Some(0.1),
            Status::Resolved => Some(0.2),
            Status::Connecting => Some(0.3),
            Status::Connected => Some(0.4),
            Status::ProxyNegotiating => Some(0.5),
            Status::ProxyNegotiated => Some(0.6),
            Status::TlsHandshaking => Some(0.7),
            Status::TlsHandshaked => Some(0.8),
            Status::Complete => Some(0.9),
            Status::Failure | Status::Redirect | Status::Success | Status::Input => Some(1.0),
            Status::New | Status::SessionRestored => None,
        }
    }

    pub fn is_loading(&self) -> bool {
        match self.progress_fraction() {
            Some(progress_fraction) => progress_fraction < 1.0,
            None => false,
        }
    }

    pub fn meta_title(&self) -> GString {
        self.meta.title()
    }

    pub fn gobject(&self) -> &Box {
        &self.widget.gobject()
    }

    // Private helpers @TODO move outside
    fn load_gemini(&self, uri: Uri) {
        // Use local namespaces @TODO
        // use gemini::client::response::

        // Init shared objects (async)
        let action_page_open = self.action_page_open.clone();
        let action_page_reload = self.action_page_reload.clone();
        let action_update = self.action_update.clone();
        let content = self.content.clone();
        let id = self.id.to_variant();
        let input = self.input.clone();
        let meta = self.meta.clone();
        let url = uri.clone().to_str();

        // Init socket
        let client = SocketClient::new();

        client.set_protocol(SocketProtocol::Tcp);
        client.set_tls_validation_flags(TlsCertificateFlags::INSECURE);
        client.set_tls(true);

        // Listen for connection status updates
        client.connect_event({
            let action_update = action_update.clone();
            let id = id.clone();
            let meta = meta.clone();
            move |_, event, _, _| {
                meta.set_status(match event {
                    SocketClientEvent::Resolving => Status::Resolving,
                    SocketClientEvent::Resolved => Status::Resolved,
                    SocketClientEvent::Connecting => Status::Connecting,
                    SocketClientEvent::Connected => Status::Connected,
                    SocketClientEvent::ProxyNegotiating => Status::ProxyNegotiating,
                    SocketClientEvent::ProxyNegotiated => Status::ProxyNegotiated,
                    SocketClientEvent::TlsHandshaking => Status::TlsHandshaking,
                    SocketClientEvent::TlsHandshaked => Status::TlsHandshaked,
                    SocketClientEvent::Complete => Status::Complete,
                    _ => todo!(), // notice on API change
                });
                action_update.activate(Some(&id));
            }
        });

        // Create connection
        client.clone().connect_to_uri_async(
            url.clone().as_str(),
            1965,
            None::<&Cancellable>,
            move |connect| match connect {
                Ok(connection) => {
                    // Send request
                    connection.output_stream().write_bytes_async(
                        &Bytes::from(gformat!("{url}\r\n").as_bytes()),
                        Priority::DEFAULT,
                        None::<&Cancellable>,
                        move |request| match request {
                            Ok(_) => {
                                // Read meta from input stream
                                gemini::client::response::Meta::from_socket_connection_async(
                                    connection.clone(),
                                    Some(Priority::DEFAULT),
                                    None::<Cancellable>,
                                    move |result| match result
                                    {
                                        Ok(response) => {
                                            // Route by status
                                            match response.status() {
                                                // https://geminiprotocol.net/docs/protocol-specification.gmi#input-expected
                                                gemini::client::response::meta::Status::Input |
                                                gemini::client::response::meta::Status::SensitiveInput => {
                                                    // Format response
                                                    let status = Status::Input;
                                                    let title = &"Input expected";
                                                    let description = match response.data() {
                                                        Some(data) => data.value().as_str(),
                                                        None => title,
                                                    };

                                                    // Toggle input form variant
                                                    match response.status() {
                                                        gemini::client::response::meta::Status::SensitiveInput =>
                                                            input.set_new_sensitive(
                                                                action_page_open,
                                                                uri,
                                                                Some(&description),
                                                                Some(1024),
                                                            ),
                                                        _ =>
                                                            input.set_new_response(
                                                                action_page_open,
                                                                uri,
                                                                Some(&description),
                                                                Some(1024),
                                                            ),
                                                    }

                                                    // Update meta
                                                    meta.set_status(status)
                                                        .set_title(title);

                                                    // Update page
                                                    action_update.activate(Some(&id));
                                                },
                                                gemini::client::response::meta::Status::Success => {
                                                    // Route by MIME
                                                    match response.mime() {
                                                        Some(gemini::client::response::meta::Mime::TextGemini) => {
                                                            // Read entire input stream to buffer
                                                            gemini::client::response::data::Text::from_socket_connection_async(
                                                                connection,
                                                                Some(Priority::DEFAULT),
                                                                None::<Cancellable>,
                                                                move |result|{
                                                                    match result {
                                                                        Ok(buffer) => {
                                                                            // Set children component
                                                                            let text_gemini = content.to_text_gemini(
                                                                                &uri,
                                                                                &buffer.data()
                                                                            );

                                                                            let title = match text_gemini.meta_title() {
                                                                                Some(title) => title,
                                                                                None => &uri_to_title(&uri)
                                                                            };

                                                                            // Update page meta
                                                                            meta.set_status(Status::Success)
                                                                                .set_title(title);

                                                                            // Update window components
                                                                            action_update.activate(Some(&id));
                                                                        }
                                                                        Err((reason, message)) => {
                                                                            // Define common data
                                                                            let status = Status::Failure;
                                                                            let title = &"Oops";
                                                                            let description = match reason {
                                                                                gemini::client::response::data::text::Error::InputStream => match message {
                                                                                    Some(error) => gformat!("{error}"),
                                                                                    None => gformat!("Undefined connection error")
                                                                                } ,
                                                                                gemini::client::response::data::text::Error::BufferOverflow => gformat!("Buffer overflow"),
                                                                                gemini::client::response::data::text::Error::Decode => gformat!("Buffer decode error"),
                                                                            };

                                                                            // Update widget
                                                                            content
                                                                                .to_status_failure()
                                                                                .set_title(title)
                                                                                .set_description(Some(description.as_str()));

                                                                            // Update meta
                                                                            meta.set_status(status)
                                                                                .set_title(title);

                                                                            // Update window
                                                                            action_update.activate(Some(&id));
                                                                        },
                                                                    }
                                                                }
                                                            );
                                                        },
                                                        Some(
                                                            gemini::client::response::meta::Mime::ImagePng  |
                                                            gemini::client::response::meta::Mime::ImageGif  |
                                                            gemini::client::response::meta::Mime::ImageJpeg |
                                                            gemini::client::response::meta::Mime::ImageWebp
                                                        ) => {
                                                            // Final image size unknown, show loading widget
                                                            let status = content.to_status_loading(
                                                                Some(Duration::from_secs(1)) // show if download time > 1 second
                                                            );

                                                            // Asynchronously move `InputStream` data from `SocketConnection` into the local `MemoryInputStream`
                                                            // this action allows to count the bytes for loading widget and validate max size for incoming data
                                                            gemini::gio::memory_input_stream::from_socket_connection_async(
                                                                connection,
                                                                None::<Cancellable>,
                                                                Priority::DEFAULT,
                                                                0x400, // 1024 bytes per chunk, optional step for images download tracking
                                                                0xA00000, // 10M bytes max to prevent memory overflow if server play with promises
                                                                move |(_, total)| {
                                                                    // Update loading progress
                                                                    status.set_description(
                                                                        Some(&gformat!("Download: {total} bytes"))
                                                                    );
                                                                },
                                                                move |result| match result {
                                                                    Ok(memory_input_stream) => {
                                                                        Pixbuf::from_stream_async(
                                                                            &memory_input_stream,
                                                                            None::<&Cancellable>,
                                                                            move |result| {
                                                                                match result {
                                                                                    Ok(buffer) => {
                                                                                        // Update page meta
                                                                                        meta.set_status(Status::Success)
                                                                                            .set_title(uri_to_title(&uri).as_str());

                                                                                        // Update page content
                                                                                        content.to_image(&buffer);

                                                                                        // Update window components
                                                                                        action_update.activate(Some(&id));
                                                                                    }
                                                                                    Err(reason) => {
                                                                                        // Define common data
                                                                                        let status = Status::Failure;
                                                                                        let title = &"Oops";

                                                                                        // Update widget
                                                                                        content
                                                                                            .to_status_failure()
                                                                                            .set_title(title)
                                                                                            .set_description(Some(reason.message()));

                                                                                        // Update meta
                                                                                        meta.set_status(status)
                                                                                            .set_title(title);
                                                                                    }
                                                                                }
                                                                            }
                                                                        );
                                                                    },
                                                                    Err((error, reason)) => {
                                                                        // Define common data
                                                                        let status = Status::Failure;
                                                                        let title = &"Oops";
                                                                        let description = match reason {
                                                                            Some(message) => gformat!("{message}"),
                                                                            None => match error {
                                                                                gemini::gio::memory_input_stream::Error::BytesTotal => gformat!("Allowed size reached"),
                                                                                gemini::gio::memory_input_stream::Error::InputStream => gformat!("Input stream error"),
                                                                            },
                                                                        };

                                                                        // Update widget
                                                                        content
                                                                            .to_status_failure()
                                                                            .set_title(title)
                                                                            .set_description(Some(description.as_str()));

                                                                        // Update meta
                                                                        meta.set_status(status)
                                                                            .set_title(title);
                                                                    }
                                                                },
                                                            );
                                                        },
                                                        /* @TODO stream or download
                                                        Some(
                                                            ClientMime::AudioFlac | ClientMime::AudioMpeg | ClientMime::AudioOgg
                                                        ) => {
                                                            // Update page meta
                                                            meta.borrow_mut().status = Some(Status::Success);
                                                            meta.borrow_mut().title = Some(gformat!("Stream"));

                                                            // Update page content
                                                            // content.to_stream();

                                                            // Update window components
                                                            action_update.activate(Some(&id));
                                                        }, */
                                                        _ => {
                                                            // Define common data
                                                            let status = Status::Failure;
                                                            let title = &"Oops";
                                                            let description = gformat!("Content type not supported");

                                                            // Update widget
                                                            content
                                                                .to_status_failure()
                                                                .set_title(title)
                                                                .set_description(Some(description.as_str()));

                                                            // Update meta
                                                            meta.set_status(status)
                                                                .set_title(title);

                                                            // Update window
                                                            action_update.activate(Some(&id));
                                                        },
                                                    }
                                                },
                                                // https://geminiprotocol.net/docs/protocol-specification.gmi#redirection
                                                gemini::client::response::meta::Status::Redirect |
                                                gemini::client::response::meta::Status::PermanentRedirect => {
                                                    // Extract redirection URL from response data
                                                    match response.data() {
                                                        Some(unresolved_url) => {
                                                            // New URL from server MAY to be relative (according to the protocol specification),
                                                            // resolve to absolute URI gobject using current request as the base for parser:
                                                            // https://docs.gtk.org/glib/type_func.Uri.resolve_relative.html
                                                            match Uri::resolve_relative(
                                                                Some(&uri.to_string()),
                                                                &unresolved_url.value(),
                                                                UriFlags::NONE,
                                                            ) {
                                                                Ok(resolved_url) => {
                                                                    // Build valid URI from resolved URL string
                                                                    // this conversion wanted to simply exclude `query` and `fragment` later (as restricted by protocol specification)
                                                                    match Uri::parse(resolved_url.as_str(), UriFlags::NONE) {
                                                                        Ok(resolved_uri) => {
                                                                            // Client MUST prevent external redirects (by protocol specification)
                                                                            if is_external_uri(&resolved_uri, &uri) {
                                                                                // Update meta
                                                                                meta.set_status(Status::Failure)
                                                                                    .set_title(&"Oops");

                                                                                // Show placeholder with manual confirmation to continue @TODO status page?
                                                                                content.to_text_gemini(
                                                                                    &uri,
                                                                                    &gformat!(
                                                                                        "# Redirect issue\n\nExternal redirects not allowed by protocol\n\nContinue:\n\n=> {}",
                                                                                        resolved_uri.to_string()
                                                                                    )
                                                                                );
                                                                            // Client MUST limit the number of redirects they follow to 5 (by protocol specification)
                                                                            } else if meta.redirect_count() > Some(5) {
                                                                                // Update meta
                                                                                meta.set_status(Status::Failure)
                                                                                    .set_title(&"Oops");

                                                                                // Show placeholder with manual confirmation to continue @TODO status page?
                                                                                content.to_text_gemini(
                                                                                    &uri,
                                                                                    &gformat!(
                                                                                        "# Redirect issue\n\nLimit the number of redirects reached\n\nContinue:\n\n=> {}",
                                                                                        resolved_uri.to_string()
                                                                                    )
                                                                                );
                                                                            // Redirection value looks valid, create new redirect (stored in meta `Redirect` holder)
                                                                            // then call page reload action to apply it by the parental controller
                                                                            } else {
                                                                                meta.set_redirect(
                                                                                    // Skip query and fragment by protocol requirements
                                                                                    // @TODO review fragment specification
                                                                                    resolved_uri.to_string_partial(
                                                                                        UriHideFlags::FRAGMENT | UriHideFlags::QUERY
                                                                                    ),
                                                                                    // Set follow policy based on status code
                                                                                    match response.status() {
                                                                                        gemini::client::response::meta::Status::PermanentRedirect => true,
                                                                                        _ => false
                                                                                    },
                                                                                )
                                                                                    .set_status(Status::Redirect) // @TODO is this status really wanted?
                                                                                    .set_title(&"Redirect");

                                                                                // Reload page to apply redirection
                                                                                action_page_reload.activate(None);
                                                                            }
                                                                        },
                                                                        Err(reason) => {
                                                                            let status = Status::Failure;
                                                                            let title = &"Oops";

                                                                            meta.set_status(status)
                                                                                .set_title(title);

                                                                            content
                                                                                .to_status_failure()
                                                                                .set_title(title)
                                                                                .set_description(Some(reason.message()));
                                                                        }
                                                                    }
                                                                }
                                                                Err(reason) => {
                                                                    let status = Status::Failure;
                                                                    let title = &"Oops";

                                                                    meta.set_status(status)
                                                                        .set_title(title);

                                                                    content
                                                                        .to_status_failure()
                                                                        .set_title(title)
                                                                        .set_description(Some(reason.message()));
                                                                },
                                                            }
                                                        },
                                                        None => {
                                                            let status = Status::Failure;
                                                            let title = &"Oops";

                                                            meta.set_status(status)
                                                                .set_title(title);

                                                            content
                                                                .to_status_failure()
                                                                .set_title(title)
                                                                .set_description(Some("Redirection target not defined"));
                                                        },
                                                    }

                                                    action_update.activate(Some(&id));
                                                },
                                                _ => {
                                                    // Define common data
                                                    let status = Status::Failure;
                                                    let title = &"Oops";

                                                    // Update widget
                                                    content
                                                        .to_status_failure()
                                                        .set_title(title)
                                                        .set_description(Some("Status code yet not supported"));

                                                    // Update meta
                                                    meta.set_status(status)
                                                        .set_title(title);

                                                    // Update window
                                                    action_update.activate(Some(&id));
                                                }
                                            }
                                        },
                                        Err((reason, message)) => {
                                            // Define common data
                                            let status = Status::Failure;
                                            let title = &"Oops";
                                            let description = match reason {
                                                // Common
                                                gemini::client::response::meta::Error::InputStream => match message {
                                                    Some(error) => gformat!("{error}"),
                                                    None => gformat!("Input stream reading error")
                                                },
                                                gemini::client::response::meta::Error::Protocol => match message {
                                                    Some(error) => gformat!("{error}"),
                                                    None => gformat!("Incorrect protocol")
                                                },
                                                // Status
                                                gemini::client::response::meta::Error::StatusDecode => match message {
                                                    Some(error) => gformat!("{error}"),
                                                    None => gformat!("Could not detect status code")
                                                },
                                                gemini::client::response::meta::Error::StatusUndefined => match message {
                                                    Some(error) => gformat!("{error}"),
                                                    None => gformat!("Status code yet not supported")
                                                },
                                                gemini::client::response::meta::Error::StatusProtocol => match message {
                                                    Some(error) => gformat!("{error}"),
                                                    None => gformat!("Incorrect status code protocol")
                                                },
                                                // Data
                                                gemini::client::response::meta::Error::DataDecode => match message {
                                                    Some(error) => gformat!("{error}"),
                                                    None => gformat!("Incorrect data encoding")
                                                },
                                                gemini::client::response::meta::Error::DataProtocol => match message {
                                                    Some(error) => gformat!("{error}"),
                                                    None => gformat!("Incorrect data protocol")
                                                },
                                                // MIME
                                                gemini::client::response::meta::Error::MimeDecode => match message {
                                                    Some(error) => gformat!("{error}"),
                                                    None => gformat!("Incorrect MIME encoding")
                                                },
                                                gemini::client::response::meta::Error::MimeProtocol => match message {
                                                    Some(error) => gformat!("{error}"),
                                                    None => gformat!("Incorrect MIME protocol")
                                                },
                                                gemini::client::response::meta::Error::MimeUndefined => match message {
                                                    Some(error) => gformat!("{error}"),
                                                    None => gformat!("MIME type yet not supported (by library)")
                                                },
                                            };

                                            // Update widget
                                            content
                                                .to_status_failure()
                                                .set_title(title)
                                                .set_description(Some(description.as_str()));

                                            // Update meta
                                            meta.set_status(status)
                                                .set_title(title);

                                            // Update window
                                            action_update.activate(Some(&id));
                                        } // Header::from_socket_connection_async
                                    }
                                );
                            }
                            Err(reason) => {
                                // Define common data
                                let status = Status::Failure;
                                let title = &"Oops";

                                // Update widget
                                content
                                    .to_status_failure()
                                    .set_title(title)
                                    .set_description(Some(reason.message()));

                                // Update meta
                                meta.set_status(status)
                                    .set_title(title);

                                // Update window
                                action_update.activate(Some(&id));
                            },
                        },
                    );
                }
                Err(reason) => {
                    // Define common data
                    let status = Status::Failure;
                    let title = &"Oops";

                    // Update widget
                    content
                        .to_status_failure()
                        .set_title(title)
                        .set_description(Some(reason.message()));

                    // Update meta
                    meta.set_status(status)
                        .set_title(title);

                    // Update window
                    action_update.activate(Some(&id));
                },
            },
        );
    }
}

// Tools

pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = Database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    meta::migrate(tx)?;
    navigation::migrate(tx)?;

    // Success
    Ok(())
}

/// Helper function, extract readable title from [Uri](https://docs.gtk.org/glib/struct.Uri.html)
///
/// Useful as common placeholder when page title could not be detected
///
/// * this feature may be improved and moved outside @TODO
fn uri_to_title(uri: &Uri) -> GString {
    match uri.path().split('/').last() {
        Some(filename) => gformat!("{filename}"),
        None => match uri.host() {
            Some(host) => gformat!("{host}"),
            None => gformat!("Untitled"),
        },
    }
}

/// Compare `subject` with `base`
///
/// Return `false` on scheme, port or host mismatch
fn is_external_uri(subject: &Uri, base: &Uri) -> bool {
    if subject.scheme() != base.scheme() {
        return true;
    }
    if subject.port() != base.port() {
        return true;
    }
    subject.host() != base.host()
}
