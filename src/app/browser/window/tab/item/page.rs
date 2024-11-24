mod content;
mod database;
mod error;
mod input;
mod meta;
mod navigation;
mod widget;

use content::Content;
use error::Error;
use input::Input;
use meta::{Meta, Status};
use navigation::Navigation;
use widget::Widget;

use crate::app::browser::{
    window::{tab::item::Action as TabAction, Action as WindowAction},
    Action as BrowserAction,
};
use crate::Profile;
use gtk::{
    gdk::Texture,
    gdk_pixbuf::Pixbuf,
    gio::{
        Cancellable, IOStream, NetworkAddress, SocketClient, SocketClientEvent, SocketConnectable,
        SocketProtocol, TlsCertificate, TlsClientConnection,
    },
    glib::{
        gformat, Bytes, GString, Priority, Regex, RegexCompileFlags, RegexMatchFlags, Uri,
        UriFlags, UriHideFlags,
    },
    prelude::{
        CancellableExt, Cast, EditableExt, IOStreamExt, IsA, OutputStreamExt, SocketClientExt,
        TlsConnectionExt,
    },
};
use sqlite::Transaction;
use std::{cell::RefCell, rc::Rc, time::Duration};

pub struct Page {
    id: GString,
    cancellable: RefCell<Cancellable>,
    profile: Rc<Profile>,
    // Actions
    browser_action: Rc<BrowserAction>,
    tab_action: Rc<TabAction>,
    // Components
    navigation: Rc<Navigation>,
    content: Rc<Content>,
    input: Rc<Input>,
    meta: Rc<Meta>,
    widget: Rc<Widget>,
}

impl Page {
    // Constructors

    pub fn new(
        id: GString,
        profile: Rc<Profile>,
        action: (Rc<BrowserAction>, Rc<WindowAction>, Rc<TabAction>),
    ) -> Self {
        // Init components
        let content = Rc::new(Content::new((action.1.clone(), action.2.clone())));

        let navigation = Rc::new(Navigation::new(
            profile.clone(),
            (action.0.clone(), action.1.clone(), action.2.clone()),
        ));

        let input = Rc::new(Input::new());

        let widget = Rc::new(Widget::new(
            &id,
            navigation.widget().gobject(),
            content.gobject(),
            input.gobject(),
        ));

        let meta = Rc::new(Meta::new(Status::New, gformat!("New page")));

        // Done
        Self {
            cancellable: RefCell::new(Cancellable::new()),
            id,
            profile,
            // Actions
            browser_action: action.0,
            tab_action: action.2,
            // Components
            content,
            navigation,
            input,
            meta,
            widget,
        }
    }

    // Actions

    /// Toggle bookmark for current `profile` by navigation request value
    /// * return `true` on bookmark created, `false` on deleted
    pub fn bookmark(&self) -> Result<bool, Error> {
        let result = match self
            .profile
            .bookmark
            .toggle(self.navigation.request().widget().gobject().text().as_str())
        {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::Bookmark),
        };
        self.update();
        result
    }

    /// Navigate home URL (parsed from current navigation entry)
    /// * this method create new history record in memory as defined in `action_page_open` action
    pub fn home(&self) {
        if let Some(url) = self.navigation.home().url() {
            // Update with history record
            self.tab_action.load().activate(Some(&url), true);
        }
    }

    /// Navigate back in history
    /// * this method does not create new history record in memory
    pub fn history_back(&self) {
        if let Some(request) = self.navigation.history().back(true) {
            // Update navigation entry
            self.navigation
                .request()
                .widget()
                .gobject()
                .set_text(&request);

            // Load page (without history record)
            self.load(false);
        }
    }

    /// Navigate forward in history
    /// * this method does not create new history record in memory
    pub fn history_forward(&self) {
        if let Some(request) = self.navigation.history().forward(true) {
            // Update navigation entry
            self.navigation
                .request()
                .widget()
                .gobject()
                .set_text(&request);

            // Load page (without history record)
            self.load(false);
        }
    }

    /// Used as API for external reload actions
    /// * this method record history to memory buffer by default
    pub fn reload(&self) {
        self.load(true);
    }

    /// Main loader for different protocols, that routed by scheme
    /// * every protocol has it own (children) method implementation
    pub fn load(&self, history: bool) {
        /// Global limit to prevent infinitive redirects (ALL protocols)
        /// * every protocol implementation has own value checker, according to specification
        const DEFAULT_MAX_REDIRECT_COUNT: i8 = 10;

        // Reset widgets
        self.input.unset();

        // Cancel previous async operations
        let cancellable = self.cancellable.take();
        if !cancellable.is_cancelled() {
            cancellable.cancel();
        }

        // Create new cancellable
        self.cancellable.replace(Cancellable::new());

        // Create shared variant value
        let id = self.id.clone();

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
                    .request()
                    .widget()
                    .gobject()
                    .set_text(redirect.request().as_str());
            }

            // Return value from redirection holder
            redirect.request()
        } else {
            // Reset redirect counter as request value taken from user input
            self.meta.unset_redirect_count();

            // Return value from navigation entry
            self.navigation.request().widget().gobject().text()
        };

        // Add history record
        if history {
            match self.navigation.history().current() {
                Some(current) => {
                    if current != request {
                        self.navigation.history().add(request.clone(), true);
                    }
                }
                None => self.navigation.history().add(request.clone(), true),
            }
        }

        // Update
        self.meta.set_status(Status::Reload).set_title("Loading..");
        self.browser_action.update().activate(Some(&id));

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
                        let title = "Oops";

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
                        self.browser_action.update().activate(Some(&id));
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
                            // Update navigation entry
                            self.navigation
                                .request()
                                .widget()
                                .gobject()
                                .set_text(&request);

                            // Load page (without history record)
                            self.load(false);
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

                    // Update navigation entry
                    self.navigation
                        .request()
                        .widget()
                        .gobject()
                        .set_text(&request);

                    // Load page (without history record)
                    self.load(false);
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
        match database::select(transaction, app_browser_window_tab_item_id) {
            Ok(records) => {
                for record in records {
                    match database::delete(transaction, &record.id) {
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
        match database::select(transaction, app_browser_window_tab_item_id) {
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
        match database::insert(transaction, app_browser_window_tab_item_id) {
            Ok(_) => {
                let id = database::last_insert_id(transaction);

                // Delegate save action to childs
                self.meta.save(transaction, &id)?;
                self.navigation.save(transaction, &id)?;
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
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

    pub fn meta(&self) -> &Rc<Meta> {
        &self.meta
    }

    pub fn navigation(&self) -> &Rc<Navigation> {
        &self.navigation
    }

    pub fn widget(&self) -> &Rc<Widget> {
        &self.widget
    }

    // Private helpers @TODO move outside
    fn load_gemini(&self, uri: Uri) {
        // Stream wrapper for TLS connections
        fn auth(
            connection: impl IsA<IOStream>,
            connectable: impl IsA<SocketConnectable>,
            certificate: Option<TlsCertificate>,
        ) -> impl IsA<IOStream> {
            if let Some(certificate) = certificate {
                // https://geminiprotocol.net/docs/protocol-specification.gmi#the-use-of-tls
                let tls_connection =
                    TlsClientConnection::new(&connection, Some(&connectable)).unwrap(); // @TODO handle

                // https://geminiprotocol.net/docs/protocol-specification.gmi#client-certificates
                tls_connection.set_certificate(&certificate);

                // @TODO handle exceptions
                // https://geminiprotocol.net/docs/protocol-specification.gmi#closing-connections
                tls_connection.set_require_close_notify(true);

                // @TODO manual validation
                // https://geminiprotocol.net/docs/protocol-specification.gmi#tls-server-certificate-validation
                tls_connection.connect_accept_certificate(move |_, _, _| true);

                // Take encrypted I/O stream
                tls_connection.upcast::<IOStream>()
            } else {
                // Take default I/O stream
                connection.upcast::<IOStream>()
            }
        }

        // Init shared objects (async)
        let cancellable = self.cancellable.borrow().clone();
        let update = self.browser_action.update().clone();
        let tab_action = self.tab_action.clone();
        let content = self.content.clone();
        let id = self.id.clone();
        let input = self.input.clone();
        let meta = self.meta.clone();
        let url = uri.clone().to_str();

        // Init socket
        let client = SocketClient::new();
        client.set_protocol(SocketProtocol::Tcp);
        client.set_timeout(10); // @TODO

        // Return PEM string match request
        let certificate = match self
            .profile
            .identity
            .gemini(&self.navigation.request().widget().gobject().text())
        {
            Some(pem) => match TlsCertificate::from_pem(&pem) {
                Ok(certificate) => Some(certificate),
                Err(reason) => todo!("{reason}"),
            },
            None => {
                // Use unauthorized (random) TLS connection
                client.set_tls(true);
                None
            }
        };

        // Listen for connection status updates
        client.connect_event({
            let update = update.clone();
            let id = id.clone();
            let meta = meta.clone();
            move |_, event, _, stream| {
                meta.set_status(match event {
                    SocketClientEvent::Resolving => Status::Resolving,
                    SocketClientEvent::Resolved => Status::Resolved,
                    SocketClientEvent::Connecting => Status::Connecting,
                    SocketClientEvent::Connected => Status::Connected,
                    SocketClientEvent::ProxyNegotiating => Status::ProxyNegotiating,
                    SocketClientEvent::ProxyNegotiated => Status::ProxyNegotiated,
                    // This case have effect only for unauthorized (random) TLS connection
                    // * see `fn auth` above to handle custom certificates
                    SocketClientEvent::TlsHandshaking => {
                        // Handle certificate errors @TODO
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#tls-server-certificate-validation
                        stream
                            .unwrap()
                            .dynamic_cast_ref::<TlsClientConnection>()
                            .unwrap()
                            .connect_accept_certificate(|_, _, _| true);
                        Status::TlsHandshaking
                    }
                    SocketClientEvent::TlsHandshaked => Status::TlsHandshaked,
                    SocketClientEvent::Complete => Status::Complete,
                    _ => todo!(), // notice on API change
                });
                update.activate(Some(&id));
            }
        });

        // Implement shared [SocketConnectable](https://docs.gtk.org/gio/iface.SocketConnectable.html) interface
        // * required also on `auth` step ([SNI](https://geminiprotocol.net/docs/protocol-specification.gmi#server-name-indication))
        let connectable = NetworkAddress::new(&uri.host().unwrap(), 1965);

        // Create connection
        client.clone().connect_async(
            &connectable.clone(),
            Some(&cancellable.clone()),
            move |connect| match connect {
                Ok(connection) => {

                    // Encrypt stream using authorization TLS
                    let stream = auth(connection, connectable, certificate);

                    // Send request
                    stream.output_stream().write_bytes_async(
                        &Bytes::from(gformat!("{url}\r\n").as_bytes()),
                        Priority::DEFAULT,
                        Some(&cancellable.clone()),
                        move |request| match request {
                            Ok(_) => {
                                // Read meta from input stream
                                gemini::client::response::Meta::from_stream_async(
                                    stream.clone(),
                                    Some(Priority::DEFAULT),
                                    Some(cancellable.clone()),
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
                                                    let title = match response.data() {
                                                        Some(data) => data.value().as_str(),
                                                        None => "Input expected",
                                                    };

                                                    // Toggle input form variant
                                                    match response.status() {
                                                        gemini::client::response::meta::Status::SensitiveInput =>
                                                            input.set_new_sensitive(
                                                                tab_action,
                                                                uri,
                                                                Some(title),
                                                                Some(1024),
                                                            ),
                                                        _ =>
                                                            input.set_new_response(
                                                                tab_action,
                                                                uri,
                                                                Some(title),
                                                                Some(1024),
                                                            ),
                                                    }

                                                    // Update meta
                                                    meta.set_status(status)
                                                        .set_title(title);

                                                    // Update page
                                                    update.activate(Some(&id));
                                                },
                                                // https://geminiprotocol.net/docs/protocol-specification.gmi#status-20
                                                gemini::client::response::meta::Status::Success => {
                                                    // Route by MIME
                                                    match response.mime() {
                                                        Some(gemini::client::response::meta::Mime::TextGemini) => {
                                                            // Read entire input stream to buffer
                                                            gemini::client::response::data::Text::from_stream_async(
                                                                stream,
                                                                Some(Priority::DEFAULT),
                                                                Some(cancellable.clone()),
                                                                move |result|{
                                                                    match result {
                                                                        Ok(buffer) => {
                                                                            // Set children component
                                                                            let text_gemini = content.to_text_gemini(
                                                                                &uri,
                                                                                buffer.data()
                                                                            );

                                                                            let title = match text_gemini.meta_title() {
                                                                                Some(title) => title,
                                                                                None => &uri_to_title(&uri)
                                                                            };

                                                                            // Update page meta
                                                                            meta.set_status(Status::Success)
                                                                                .set_title(title);

                                                                            // Update window components
                                                                            update.activate(Some(&id));
                                                                        }
                                                                        Err((reason, message)) => {
                                                                            // Define common data
                                                                            let status = Status::Failure;
                                                                            let title = "Oops";
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
                                                                            update.activate(Some(&id));
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
                                                            gemini::gio::memory_input_stream::from_stream_async(
                                                                stream,
                                                                Some(cancellable.clone()),
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
                                                                            Some(&cancellable),
                                                                            move |result| {
                                                                                match result {
                                                                                    Ok(buffer) => {
                                                                                        // Update page meta
                                                                                        meta.set_status(Status::Success)
                                                                                            .set_title(uri_to_title(&uri).as_str());

                                                                                        // Update page content
                                                                                        content.to_image(&Texture::for_pixbuf(&buffer));

                                                                                        // Update window components
                                                                                        update.activate(Some(&id));
                                                                                    }
                                                                                    Err(reason) => {
                                                                                        // Define common data
                                                                                        let status = Status::Failure;
                                                                                        let title = "Oops";

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
                                                                        let title = "Oops";
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
                                                            update.activate(Some(&id));
                                                        }, */
                                                        _ => {
                                                            // Define common data
                                                            let status = Status::Failure;
                                                            let title = "Oops";
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
                                                            update.activate(Some(&id));
                                                        },
                                                    }
                                                },
                                                // https://geminiprotocol.net/docs/protocol-specification.gmi#status-30-temporary-redirection
                                                gemini::client::response::meta::Status::Redirect |
                                                // https://geminiprotocol.net/docs/protocol-specification.gmi#status-31-permanent-redirection
                                                gemini::client::response::meta::Status::PermanentRedirect => {
                                                    // Extract redirection URL from response data
                                                    match response.data() {
                                                        Some(unresolved_url) => {
                                                            // New URL from server MAY to be relative (according to the protocol specification),
                                                            // resolve to absolute URI gobject using current request as the base for parser:
                                                            // https://docs.gtk.org/glib/type_func.Uri.resolve_relative.html
                                                            match Uri::resolve_relative(
                                                                Some(&uri.to_string()),
                                                                unresolved_url.value(),
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
                                                                                    .set_title("Oops");

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
                                                                                    .set_title("Oops");

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
                                                                                    matches!(response.status(), gemini::client::response::meta::Status::PermanentRedirect),
                                                                                )
                                                                                    .set_status(Status::Redirect) // @TODO is this status really wanted?
                                                                                    .set_title("Redirect");

                                                                                // Reload page to apply redirection (without history record request)
                                                                                tab_action.load().activate(None, false);
                                                                            }
                                                                        },
                                                                        Err(reason) => {
                                                                            let status = Status::Failure;
                                                                            let title = "Oops";

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
                                                                    let title = "Oops";

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
                                                            let title = "Oops";

                                                            meta.set_status(status)
                                                                .set_title(title);

                                                            content
                                                                .to_status_failure()
                                                                .set_title(title)
                                                                .set_description(Some("Redirection target not defined"));
                                                        },
                                                    }

                                                    update.activate(Some(&id));
                                                },
                                                // https://geminiprotocol.net/docs/protocol-specification.gmi#status-60
                                                gemini::client::response::meta::Status::CertificateRequest |
                                                // https://geminiprotocol.net/docs/protocol-specification.gmi#status-61-certificate-not-authorized
                                                gemini::client::response::meta::Status::CertificateUnauthorized |
                                                // https://geminiprotocol.net/docs/protocol-specification.gmi#status-62-certificate-not-valid
                                                gemini::client::response::meta::Status::CertificateInvalid => {
                                                    // Define common data
                                                    let status = Status::Success;
                                                    let title = "Identity";

                                                    // Update widget
                                                    content
                                                        .to_status_identity()
                                                        .set_title(title)
                                                        .set_description(match response.data() {
                                                            Some(data) => Some(data.value().as_str()),
                                                            None => match response.status() {
                                                                gemini::client::response::meta::Status::CertificateUnauthorized => Some("Certificate not authorized"),
                                                                gemini::client::response::meta::Status::CertificateInvalid => Some("Certificate not valid"),
                                                                _ => Some("Client certificate required")
                                                            },
                                                        });

                                                    // Update meta
                                                    meta.set_status(status)
                                                        .set_title(title);

                                                    // Update window
                                                    update.activate(Some(&id));
                                                }
                                                _ => {
                                                    // Define common data
                                                    let status = Status::Failure;
                                                    let title = "Oops";

                                                    // Update widget
                                                    content
                                                        .to_status_failure()
                                                        .set_title(title)
                                                        .set_description(Some(match response.data() {
                                                            Some(data) => data.value().as_str(),
                                                            None => "Status code yet not supported", // @TODO
                                                        }));

                                                    // Update meta
                                                    meta.set_status(status)
                                                        .set_title(title);

                                                    // Update window
                                                    update.activate(Some(&id));
                                                }
                                            }
                                        },
                                        Err((reason, message)) => {
                                            // Define common data
                                            let status = Status::Failure;
                                            let title = "Oops";
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
                                            update.activate(Some(&id));
                                        } // Header::from_socket_connection_async
                                    }
                                );
                            }
                            Err(reason) => {
                                // Define common data
                                let status = Status::Failure;
                                let title = "Oops";

                                // Update widget
                                content
                                    .to_status_failure()
                                    .set_title(title)
                                    .set_description(Some(reason.message()));

                                // Update meta
                                meta.set_status(status)
                                    .set_title(title);

                                // Update window
                                update.activate(Some(&id));
                            },
                        },
                    );
                }
                Err(reason) => {
                    // Define common data
                    let status = Status::Failure;
                    let title = "Oops";

                    // Update widget
                    content
                        .to_status_failure()
                        .set_title(title)
                        .set_description(Some(reason.message()));

                    // Update meta
                    meta.set_status(status)
                        .set_title(title);

                    // Update window
                    update.activate(Some(&id));
                },
            },
        );
    }
}

// Tools

pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = database::init(tx) {
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
    let title = GString::from(uri.path().split('/').last().unwrap_or_default());
    if title.is_empty() {
        match uri.host() {
            Some(host) => gformat!("{host}"),
            None => gformat!("Untitled"),
        }
    } else {
        title
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
