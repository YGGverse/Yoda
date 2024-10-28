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
        RegexMatchFlags, Uri, UriFlags,
    },
    prelude::{
        ActionExt, IOStreamExt, OutputStreamExt, SocketClientExt, StaticVariantType, ToVariant,
    },
    Box,
};
use sqlite::Transaction;
use std::{cell::RefCell, sync::Arc};

pub struct Page {
    id: GString,
    // Actions
    action_page_open: SimpleAction,
    action_tab_page_navigation_reload: SimpleAction,
    action_update: SimpleAction,
    // Components
    navigation: Arc<Navigation>,
    content: Arc<Content>,
    input: Arc<Input>,
    // Extras
    meta: Arc<RefCell<Meta>>,
    // GTK
    widget: Arc<Widget>,
}

impl Page {
    // Construct
    pub fn new_arc(
        id: GString,
        action_tab_open: SimpleAction,
        action_tab_page_navigation_base: SimpleAction,
        action_tab_page_navigation_history_back: SimpleAction,
        action_tab_page_navigation_history_forward: SimpleAction,
        action_tab_page_navigation_reload: SimpleAction,
        action_update: SimpleAction,
    ) -> Arc<Self> {
        // Init local actions
        let action_page_open =
            SimpleAction::new(&uuid_string_random(), Some(&String::static_variant_type()));

        // Init components
        let content = Arc::new(Content::new(
            action_tab_open.clone(),
            action_page_open.clone(),
        ));

        let navigation = Navigation::new_arc(
            action_tab_page_navigation_base.clone(),
            action_tab_page_navigation_history_back.clone(),
            action_tab_page_navigation_history_forward.clone(),
            action_tab_page_navigation_reload.clone(),
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

        // Init async mutable Meta object
        let meta = Arc::new(RefCell::new(Meta::new()));

        // Init events
        action_page_open.connect_activate({
            let navigation = navigation.clone();
            let action_tab_page_navigation_reload = action_tab_page_navigation_reload.clone();
            move |_, request| {
                // Update request
                navigation.set_request_text(
                    request
                        .expect("Parameter required for `page.open` action")
                        .get::<String>()
                        .expect("Parameter does not match `String`")
                        .as_str(),
                );

                // Reload page
                action_tab_page_navigation_reload.activate(None);
            }
        });

        // Return activated structure
        Arc::new(Self {
            id,
            // Actions
            action_page_open,
            action_tab_page_navigation_reload,
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
    pub fn navigation_request_grab_focus(&self) {
        self.navigation.request_grab_focus();
    }

    pub fn navigation_base(&self) {
        if let Some(url) = self.navigation.base_url() {
            // Update with history record
            self.action_page_open.activate(Some(&url.to_variant()));
        }
    }

    pub fn navigation_history_back(&self) {
        if let Some(request) = self.navigation.history_back(true) {
            // Update
            self.navigation.set_request_text(&request);

            // Reload page
            self.action_tab_page_navigation_reload.activate(None);
        }
    }

    pub fn navigation_history_forward(&self) {
        if let Some(request) = self.navigation.history_forward(true) {
            // Update
            self.navigation.set_request_text(&request);

            // Reload page
            self.action_tab_page_navigation_reload.activate(None);
        }
    }

    pub fn navigation_reload(&self) {
        // Reset widgets
        self.input.unset();

        // Init shared objects to not spawn a lot
        let request_text = self.navigation.request_text();
        let id = self.id.to_variant();

        // Update
        self.meta.replace(Meta {
            status: Some(Status::Reload),
            title: Some(gformat!("Loading..")),
            //description: None,
        });

        self.action_update.activate(Some(&id));

        // Route by request
        match Uri::parse(&request_text, UriFlags::NONE) {
            Ok(uri) => {
                // Route by scheme
                match uri.scheme().as_str() {
                    "file" => todo!(),
                    "gemini" => self.load_gemini(uri), // @TODO
                    scheme => {
                        // Define common data
                        let status = Status::Failure;
                        let title = gformat!("Oops");
                        let description = gformat!("Protocol `{scheme}` not supported");

                        // Update widget
                        self.content
                            .set_status_failure(Some(title.as_str()), Some(description.as_str()));

                        // Update meta
                        self.meta.replace(Meta {
                            status: Some(status),
                            title: Some(title),
                            //description: Some(description),
                        });

                        // Update window
                        self.action_update.activate(Some(&id));
                    }
                }
            }
            Err(_) => {
                // Try interpret URI manually
                if Regex::match_simple(
                    r"^[^\/\s]+\.[\w]{2,}",
                    request_text.clone(),
                    RegexCompileFlags::DEFAULT,
                    RegexMatchFlags::DEFAULT,
                ) {
                    // Seems request contain some host, try append default scheme
                    let request_text = gformat!("gemini://{request_text}");
                    // Make sure new request conversable to valid URI
                    match Uri::parse(&request_text, UriFlags::NONE) {
                        Ok(_) => {
                            // Update
                            self.navigation.set_request_text(&request_text);

                            // Reload page
                            self.action_tab_page_navigation_reload.activate(None);
                        }
                        Err(_) => {
                            // @TODO any action here?
                        }
                    }
                } else {
                    // Plain text given, make search request to default provider
                    let request_text = gformat!(
                        "gemini://tlgs.one/search?{}",
                        Uri::escape_string(&request_text, None, false)
                    );

                    // Update
                    self.navigation.set_request_text(&request_text);

                    // Reload page
                    self.action_tab_page_navigation_reload.activate(None);
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
        match Database::records(transaction, app_browser_window_tab_item_id) {
            Ok(records) => {
                for record in records {
                    // Delegate restore action to the item childs
                    self.navigation.restore(transaction, &record.id)?;
                }
            }
            Err(e) => return Err(e.to_string()),
        }

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
                self.navigation.save(transaction, &id)?;
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    // Setters
    pub fn set_navigation_request_text(&self, value: &str) {
        self.navigation.set_request_text(value);
    }

    // Getters
    pub fn progress_fraction(&self) -> Option<f64> {
        // Interpret status to progress fraction
        match self.meta.borrow().status {
            Some(Status::Reload) => Some(0.0),
            Some(Status::Resolving) => Some(0.1),
            Some(Status::Resolved) => Some(0.2),
            Some(Status::Connecting) => Some(0.3),
            Some(Status::Connected) => Some(0.4),
            Some(Status::ProxyNegotiating) => Some(0.5),
            Some(Status::ProxyNegotiated) => Some(0.6),
            Some(Status::TlsHandshaking) => Some(0.7),
            Some(Status::TlsHandshaked) => Some(0.8),
            Some(Status::Complete) => Some(0.9),
            Some(Status::Failure | Status::Redirect | Status::Success | Status::Input) => Some(1.0),
            _ => None,
        }
    }

    pub fn is_loading(&self) -> bool {
        match self.progress_fraction() {
            Some(progress_fraction) => progress_fraction < 1.0,
            None => false,
        }
    }

    pub fn meta_title(&self) -> Option<GString> {
        self.meta.borrow().title.clone()
    }

    pub fn gobject(&self) -> &Box {
        &self.widget.gobject()
    }

    // Tools
    pub fn migrate(tx: &Transaction) -> Result<(), String> {
        // Migrate self components
        if let Err(e) = Database::init(tx) {
            return Err(e.to_string());
        }

        // Delegate migration to childs
        Navigation::migrate(tx)?;

        // Success
        Ok(())
    }

    // Private helpers @TODO
    fn load_gemini(&self, uri: Uri) {
        // Use local namespaces
        use gemini::client::response::{
            body::Error as BodyError,
            header::{Error as HeaderError, Mime as ClientMime, Status as ClientStatus},
            Body, Header,
        };

        // Init shared objects (async)
        let action_page_open = self.action_page_open.clone();
        let action_update = self.action_update.clone();
        let content = self.content.clone();
        let id = self.id.to_variant();
        let input = self.input.clone();
        let meta = self.meta.clone();
        let navigation = self.navigation.clone();
        let url = uri.clone().to_str();

        // Add history record
        match navigation.history_current() {
            Some(current) => {
                if current != url {
                    navigation.history_add(url.clone());
                }
            }
            None => navigation.history_add(url.clone()),
        }

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
                meta.borrow_mut().status = Some(match event {
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
                                // Read header from input stream
                                Header::from_socket_connection_async(
                                    connection.clone(),
                                    Some(Priority::DEFAULT),
                                    None::<Cancellable>,
                                    move |result| match result
                                    {
                                        Ok(header) => {
                                            // Route by status
                                            match header.status() {
                                                // https://geminiprotocol.net/docs/protocol-specification.gmi#input-expected
                                                ClientStatus::Input | ClientStatus::SensitiveInput => {
                                                    // Format response
                                                    let status = Status::Input;
                                                    let title = gformat!("Input expected");
                                                    let description = match header.meta() {
                                                        Some(meta) => match meta.to_gstring() {
                                                            Ok(value) => value,
                                                            Err(_) => title.clone(),
                                                        },
                                                        None => title.clone(),
                                                    };

                                                    // Make input form
                                                    match header.status() {
                                                        ClientStatus::SensitiveInput =>
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
                                                    meta.replace(Meta {
                                                        status: Some(status),
                                                        title: Some(title),
                                                        //description: Some(description),
                                                    });

                                                    // Update page
                                                    action_update.activate(Some(&id));
                                                },
                                                ClientStatus::Success => {
                                                    // Route by MIME
                                                    match header.mime() {
                                                        Some(ClientMime::TextGemini) => {
                                                            // Read entire input stream to buffer
                                                            Body::from_socket_connection_async(
                                                                connection,
                                                                move |result|{
                                                                    match result {
                                                                        Ok(buffer) => {
                                                                            // Update page meta
                                                                            meta.borrow_mut().status = Some(Status::Success);
                                                                            meta.borrow_mut().title = content.set_text_gemini(
                                                                                &uri,
                                                                                &match GString::from_utf8(buffer.to_utf8()) {
                                                                                    Ok(gemtext) => gemtext,
                                                                                    Err(_) => todo!()
                                                                                }
                                                                            );

                                                                            // Update window components
                                                                            action_update.activate(Some(&id));
                                                                        }
                                                                        Err((reason, message)) => {
                                                                            // Define common data
                                                                            let status = Status::Failure;
                                                                            let title = gformat!("Oops");
                                                                            let description = match reason {
                                                                                BodyError::InputStreamRead => match message {
                                                                                    Some(error) => gformat!("{error}"),
                                                                                    None => gformat!("Undefined connection error")
                                                                                } ,
                                                                                BodyError::BufferOverflow => gformat!("Buffer overflow"),
                                                                                BodyError::Decode => gformat!("Buffer decode error"),
                                                                            };

                                                                            // Update widget
                                                                            content.set_status_failure(
                                                                                Some(title.as_str()),
                                                                                Some(description.as_str())
                                                                            );

                                                                            // Update meta
                                                                            meta.replace(Meta {
                                                                                status: Some(status),
                                                                                title: Some(title),
                                                                                //description: Some(description),
                                                                            });

                                                                            // Update window
                                                                            action_update.activate(Some(&id));
                                                                        },
                                                                    }
                                                                }
                                                            );
                                                        },
                                                        Some(
                                                            ClientMime::ImagePng  | ClientMime::ImageGif |
                                                            ClientMime::ImageJpeg | ClientMime::ImageWebp
                                                        ) => {
                                                            // Init loading placeholder
                                                            /* @TODO count bytes on download
                                                            let title = gformat!("Loading..");
                                                            let description = gformat!(""); // collect totals here, invisible on start

                                                            content.set_status_loading(
                                                                Some(&title),
                                                                Some(&description)
                                                            ); */

                                                            match Pixbuf::from_stream( // @TODO async
                                                                &connection.input_stream(),
                                                                None::<&Cancellable>,
                                                            ) {
                                                                Ok(buffer) => {
                                                                    // Update page meta
                                                                    meta.borrow_mut().status = Some(Status::Success);
                                                                    meta.borrow_mut().title = Some(gformat!("Image"));

                                                                    // Update page content
                                                                    content.set_image(&buffer);

                                                                    // Update window components
                                                                    action_update.activate(Some(&id));
                                                                }
                                                                Err(reason) => { // Pixbuf::from_stream
                                                                    // Define common data
                                                                    let status = Status::Failure;
                                                                    let title = gformat!("Oops");
                                                                    let description = gformat!("{}", reason.message());

                                                                    // Update widget
                                                                    content.set_status_failure(
                                                                        Some(title.as_str()),
                                                                        Some(description.as_str())
                                                                    );

                                                                    // Update meta
                                                                    meta.replace(Meta {
                                                                        status: Some(status),
                                                                        title: Some(title),
                                                                        //description: Some(description),
                                                                    });
                                                                }
                                                            }
                                                        },
                                                        /* @TODO stream or download
                                                        Some(
                                                            ClientMime::AudioFlac | ClientMime::AudioMpeg | ClientMime::AudioOgg
                                                        ) => {
                                                            // Update page meta
                                                            meta.borrow_mut().status = Some(Status::Success);
                                                            meta.borrow_mut().title = Some(gformat!("Stream"));

                                                            // Update page content
                                                            // content.set_stream();

                                                            // Update window components
                                                            action_update.activate(Some(&id));
                                                        }, */
                                                        _ => {
                                                            // Define common data
                                                            let status = Status::Failure;
                                                            let title = gformat!("Oops");
                                                            let description =
                                                                gformat!("Content type not supported");

                                                            // Update widget
                                                            content.set_status_failure(
                                                                Some(title.as_str()),
                                                                Some(description.as_str()),
                                                            );

                                                            // Update meta
                                                            meta.replace(Meta {
                                                                status: Some(status),
                                                                title: Some(title),
                                                                //description: Some(description),
                                                            });

                                                            // Update window
                                                            action_update.activate(Some(&id));
                                                        },
                                                    }
                                                },
                                                // https://geminiprotocol.net/docs/protocol-specification.gmi#redirection
                                                ClientStatus::Redirect | ClientStatus::PermanentRedirect => {

                                                    // @TODO ClientStatus::TemporaryRedirect

                                                    // Update meta
                                                    meta.borrow_mut().status = Some(Status::Redirect);
                                                    meta.borrow_mut().title = Some(gformat!("Redirect"));

                                                    // Build gemtext message for manual redirection @TODO use template?
                                                    match header.meta() {
                                                        Some(meta) => {
                                                            let _ = content.set_text_gemini(
                                                                &uri,
                                                                &match meta.to_gstring() {
                                                                    Ok(url) => gformat!(
                                                                        "# Redirect\n\nAuto-follow not implemented, click on link below to continue\n\n=> {url}"
                                                                    ),
                                                                    Err(_) => gformat!(
                                                                        "# Redirect\n\nProvider request redirect but not provided any target."
                                                                    )
                                                                }
                                                            );
                                                        },
                                                        None => content.set_status_failure(
                                                            Some(&"Oops"),
                                                            Some(&"Could not parse redirect meta")
                                                        ),
                                                    }

                                                    action_update.activate(Some(&id));
                                                },
                                            }
                                        },
                                        Err((reason, message)) => {
                                            // Define common data
                                            let status = Status::Failure;
                                            let title = gformat!("Oops");
                                            let description = match reason {
                                                HeaderError::Buffer => match message {
                                                    Some(error) => gformat!("{error}"),
                                                    None => gformat!("Buffer error")
                                                },
                                                HeaderError::InputStream => match message {
                                                    Some(error) => gformat!("{error}"),
                                                    None => gformat!("Input stream reading error")
                                                },
                                                HeaderError::Protocol => match message {
                                                    Some(error) => gformat!("{error}"),
                                                    None => gformat!("Incorrect protocol")
                                                },
                                                HeaderError::StatusDecode => match message {
                                                    Some(error) => gformat!("{error}"),
                                                    None => gformat!("Could not detect status code")
                                                },
                                                HeaderError::StatusUndefined => match message {
                                                    Some(error) => gformat!("{error}"),
                                                    None => gformat!("Status code yet not supported")
                                                },
                                            };

                                            // Update widget
                                            content.set_status_failure(
                                                Some(title.as_str()),
                                                Some(description.as_str())
                                            );

                                            // Update meta
                                            meta.replace(Meta {
                                                status: Some(status),
                                                title: Some(title),
                                                //description: Some(description),
                                            });

                                            // Update window
                                            action_update.activate(Some(&id));
                                        } // Header::from_socket_connection_async
                                    }
                                );
                            }
                            Err(reason) => {
                                // Define common data
                                let status = Status::Failure;
                                let title = gformat!("Oops");
                                let description = gformat!("Request error: {}", reason.message());

                                // Update widget
                                content.set_status_failure(
                                    Some(title.as_str()),
                                    Some(description.as_str())
                                );

                                // Update meta
                                meta.replace(Meta {
                                    status: Some(status),
                                    title: Some(title),
                                    //description: Some(description),
                                });

                                // Update window
                                action_update.activate(Some(&id));
                            }, // OutputStream::write_bytes_async
                        },
                    );
                }
                Err(reason) => {
                    // Define common data
                    let status = Status::Failure;
                    let title = gformat!("Oops");
                    let description = gformat!("Connection error: {}", reason.message());

                    // Update widget
                    content.set_status_failure(
                        Some(title.as_str()),
                        Some(description.as_str())
                    );

                    // Update meta
                    meta.replace(Meta {
                        status: Some(status),
                        title: Some(title),
                        //description: Some(description),
                    });

                    // Update window
                    action_update.activate(Some(&id));
                }, // SocketClient::connect_to_uri_async
            },
        );
    }
}
