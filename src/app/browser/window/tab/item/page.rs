mod content;
mod meta;
mod navigation;

use content::Content;
use meta::{Meta, Mime, Status};
use navigation::Navigation;

use gtk::{
    gio::{
        Cancellable, SimpleAction, SimpleActionGroup, SocketClient, SocketProtocol,
        TlsCertificateFlags,
    },
    glib::{gformat, GString, Priority, Regex, RegexCompileFlags, RegexMatchFlags, Uri, UriFlags},
    prelude::{
        ActionExt, ActionMapExt, BoxExt, IOStreamExt, InputStreamExtManual, OutputStreamExtManual,
        SocketClientExt, StaticVariantType, ToVariant, WidgetExt,
    },
    Box, Orientation,
};
use std::{cell::RefCell, path::Path, sync::Arc};

pub struct Page {
    // GTK
    widget: Box,
    // Actions
    action_page_open: Arc<SimpleAction>,
    action_tab_page_navigation_reload: Arc<SimpleAction>,
    action_update: Arc<SimpleAction>,
    // Components
    navigation: Arc<Navigation>,
    content: Arc<Content>,
    // Extras
    meta: Arc<RefCell<Meta>>,
}

impl Page {
    // Construct
    pub fn new(
        name: GString,
        navigation_request_text: Option<GString>,
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_update: Arc<SimpleAction>,
    ) -> Arc<Self> {
        // Init actions
        let action_page_open = Arc::new(SimpleAction::new(
            "open",
            Some(&String::static_variant_type()),
        ));

        // Init action group
        let action_group = SimpleActionGroup::new();
        action_group.add_action(action_page_open.as_ref());

        // Init components
        let content = Arc::new(Content::new(action_page_open.clone()));
        let navigation = Arc::new(Navigation::new(
            navigation_request_text,
            action_tab_page_navigation_base.clone(),
            action_tab_page_navigation_history_back.clone(),
            action_tab_page_navigation_history_forward.clone(),
            action_tab_page_navigation_reload.clone(),
            action_update.clone(),
        ));

        // Init widget
        let widget = Box::builder()
            .orientation(Orientation::Vertical)
            .name(name)
            .build();

        widget.append(navigation.widget());
        widget.append(content.widget());

        widget.insert_action_group("page", Some(&action_group));

        // Init async mutable Meta object
        let meta = Arc::new(RefCell::new(Meta::new()));

        // Init events
        action_page_open.connect_activate({
            let navigation = navigation.clone();
            let action_tab_page_navigation_reload = action_tab_page_navigation_reload.clone();
            move |_, request| {
                // Convert to GString
                let request = GString::from(
                    request
                        .expect("Parameter required for `page.open` action")
                        .get::<String>()
                        .expect("Parameter does not match `String`"),
                );

                // Update
                navigation.set_request_text(&request);

                // Reload page
                action_tab_page_navigation_reload.activate(None);
            }
        });

        // Return activated structure
        Arc::new(Self {
            // GTK
            widget,
            // Actions
            action_page_open,
            action_tab_page_navigation_reload,
            action_update,
            // Components
            content,
            navigation,
            // Extras
            meta,
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
        // Init globals
        let request_text = self.navigation.request_text();

        // Init shared objects for async access
        let navigation = self.navigation.clone();
        let content = self.content.clone();
        let meta = self.meta.clone();
        let action_update = self.action_update.clone();

        // Update
        meta.borrow_mut().mime = None;
        meta.borrow_mut().status = Some(Status::Reload);
        meta.borrow_mut().title = Some(gformat!("Loading.."));
        meta.borrow_mut().description = None;

        action_update.activate(None);

        /*let _uri = */
        match Uri::parse(&request_text, UriFlags::NONE) {
            Ok(uri) => {
                // Route request by scheme
                match uri.scheme().as_str() {
                    "file" => {
                        todo!()
                    }
                    "gemini" => {
                        // Get host
                        let host = match uri.host() {
                            Some(host) => host,
                            None => todo!(),
                        };

                        // Update
                        meta.borrow_mut().status = Some(Status::Prepare);
                        meta.borrow_mut().description = Some(gformat!("Connect {host}.."));

                        action_update.activate(None);

                        // Create new connection
                        let cancellable = Cancellable::new();
                        let client = SocketClient::new();

                        client.set_timeout(10);
                        client.set_tls(true);
                        client.set_tls_validation_flags(TlsCertificateFlags::INSECURE);
                        client.set_protocol(SocketProtocol::Tcp);

                        client.connect_to_uri_async(
                            &uri.to_str(),
                            1965,
                            Some(&cancellable.clone()),
                            move |result| match result {
                                Ok(connection) => {
                                    // Update
                                    meta.borrow_mut().status = Some(Status::Connect);
                                    meta.borrow_mut().description = Some(gformat!("Connected to {host}.."));

                                    action_update.activate(None);

                                    // Send request
                                    connection.output_stream().write_all_async(
                                        gformat!("{}\r\n", &uri.to_str()),
                                        Priority::DEFAULT,
                                        Some(&cancellable.clone()),
                                        move |result| match result {
                                            Ok(_) => {
                                                // Update
                                                meta.borrow_mut().status = Some(Status::Request);
                                                meta.borrow_mut().description = Some(gformat!("Request data from {host}.."));

                                                action_update.activate(None);

                                                // Read response
                                                connection.input_stream().read_all_async(
                                                    vec![0; 0xfffff], // 1Mb @TODO
                                                    Priority::DEFAULT,
                                                    Some(&cancellable.clone()),
                                                    move |result| match result {
                                                        Ok(response) => {
                                                            match GString::from_utf8_until_nul(
                                                                response.0,
                                                            ) {
                                                                Ok(data) => {
                                                                    // Format response
                                                                    meta.borrow_mut().status = Some(Status::Response);
                                                                    meta.borrow_mut().description = Some(host);
                                                                    meta.borrow_mut().title = Some(uri.path());

                                                                    action_update.activate(None);

                                                                    // Try create short base for title
                                                                    let path = uri.path();
                                                                    let path = Path::new(&path);
                                                                    if let Some(base) = path.file_name() {
                                                                        if let Some(base_str) = base.to_str() {
                                                                            meta.borrow_mut().title = Some(GString::from(base_str));
                                                                        }
                                                                    }

                                                                    // Parse response @TODO read bytes
                                                                    let parts = Regex::split_simple(
                                                                        r"^(\d+)?\s([\w]+\/[\w]+)?(.*)?",
                                                                        &data,
                                                                        RegexCompileFlags::DEFAULT,
                                                                        RegexMatchFlags::DEFAULT,
                                                                    );

                                                                    // https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes
                                                                    match parts.get(1) {
                                                                        Some(code) => match code.as_str() {
                                                                            "20" => {
                                                                                match parts.get(2) {
                                                                                    Some(mime) => match mime.as_str() {
                                                                                        "text/gemini" => {
                                                                                            // Update meta
                                                                                            meta.borrow_mut().mime = Some(Mime::TextGemini);

                                                                                            // Update data
                                                                                            match parts.get(4) {
                                                                                                Some(source) => {
                                                                                                    meta.borrow_mut().status = Some(Status::Success);

                                                                                                    // This content type may return parsed title
                                                                                                    let result = content.reset(content::Mime::TextGemini, &uri, &source);
                                                                                                    meta.borrow_mut().title = result.title.clone();

                                                                                                    // Add new history record
                                                                                                    let request = uri.to_str();

                                                                                                    match navigation.history_current() {
                                                                                                        Some(current) => {
                                                                                                            if current != request {
                                                                                                                navigation.history_add(request);
                                                                                                            }
                                                                                                        }
                                                                                                        None => navigation.history_add(request),
                                                                                                    }

                                                                                                    // Update window components
                                                                                                    action_update.activate(None);
                                                                                                },
                                                                                                None => todo!(),
                                                                                            }
                                                                                        },
                                                                                        "text/plain" => {
                                                                                            meta.borrow_mut().status = Some(Status::Success);
                                                                                            meta.borrow_mut().mime = Some(Mime::TextPlain);

                                                                                            action_update.activate(None);
                                                                                            todo!()
                                                                                        },
                                                                                        _ => {
                                                                                            meta.borrow_mut().status = Some(Status::Failure);
                                                                                            meta.borrow_mut().title = Some(gformat!("Oops"));
                                                                                            meta.borrow_mut().description = Some(gformat!("Content {mime} not supported"));

                                                                                            action_update.activate(None);
                                                                                        },
                                                                                    }
                                                                                    None => todo!(),
                                                                                };
                                                                            },
                                                                            // Redirect (@TODO implement limits to auto-redirect)
                                                                            "31" => {
                                                                                // Update meta
                                                                                meta.borrow_mut().status = Some(Status::Redirect);
                                                                                meta.borrow_mut().mime = Some(Mime::TextGemini);
                                                                                meta.borrow_mut().title = Some(gformat!("Redirect"));

                                                                                action_update.activate(None);

                                                                                // Select widget
                                                                                match parts.get(3) {
                                                                                    Some(source) => {
                                                                                        let _ = content.reset(
                                                                                            content::Mime::TextGemini,
                                                                                            &uri,
                                                                                            // @TODO use template file
                                                                                            &gformat!("# Redirect\n\nAuto-follow disabled, click on link below to continue\n\n=> {source}")
                                                                                        );
                                                                                    },
                                                                                    None => todo!(),
                                                                                }
                                                                            },
                                                                            // @TODO
                                                                            _ => {
                                                                                // Update
                                                                                meta.borrow_mut().status = Some(Status::Failure);
                                                                                meta.borrow_mut().title = Some(gformat!("Oops"));
                                                                                meta.borrow_mut().description = Some(gformat!("Status {code} not supported"));

                                                                                action_update.activate(None);
                                                                            },
                                                                        }
                                                                        None => todo!(),
                                                                    };
                                                                }
                                                                Err(e) => {
                                                                    meta.borrow_mut().status = Some(Status::Failure);
                                                                    meta.borrow_mut().title = Some(gformat!("Oops"));
                                                                    meta.borrow_mut().description = Some(gformat!("Failed to read buffer data: {e}"));

                                                                    action_update.activate(None);
                                                                }
                                                            }

                                                            // Close connection
                                                            if let Err(e) = connection.close(Some(&cancellable)) {
                                                                todo!("Error closing connection: {:?}", e);
                                                            }
                                                        }
                                                        Err(e) => {
                                                            // Update
                                                            meta.borrow_mut().status = Some(Status::Failure);
                                                            meta.borrow_mut().title = Some(gformat!("Oops"));
                                                            meta.borrow_mut().description = Some(gformat!("Failed to read response: {:?}", e));

                                                            action_update.activate(None);

                                                            // Close connection
                                                            if let Err(e) = connection.close(Some(&cancellable)) {
                                                                todo!("Error closing response connection: {:?}", e);
                                                            }
                                                        }
                                                    },
                                                );
                                            }
                                            Err(e) => {
                                                // Update
                                                meta.borrow_mut().status = Some(Status::Failure);
                                                meta.borrow_mut().title = Some(gformat!("Oops"));
                                                meta.borrow_mut().description = Some(gformat!("Failed to read request: {:?}", e));

                                                action_update.activate(None);

                                                // Close connection
                                                if let Err(e) = connection.close(Some(&cancellable)) {
                                                    todo!("Error closing request connection: {:?}", e);
                                                }
                                            }
                                        },
                                    );
                                }
                                Err(e) => {
                                    // Update
                                    meta.borrow_mut().status = Some(Status::Failure);
                                    meta.borrow_mut().title = Some(gformat!("Oops"));
                                    meta.borrow_mut().description = Some(gformat!("Failed to connect: {:?}", e));

                                    action_update.activate(None);
                                }
                            },
                        );
                    }
                    /* @TODO
                    "nex" => {}
                    */
                    scheme => {
                        // Update
                        meta.borrow_mut().status = Some(Status::Failure);
                        meta.borrow_mut().title = Some(gformat!("Oops"));
                        meta.borrow_mut().description =
                            Some(gformat!("Protocol {scheme} not supported"));

                        action_update.activate(None);
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
                    // Make sure new request conversible to valid URI
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
        };
    }

    pub fn update(&self) {
        // Interpret status to progress fraction
        let progress_fraction = match self.meta.borrow().status {
            Some(Status::Prepare | Status::Reload) => Some(0.0),
            Some(Status::Connect) => Some(0.25),
            Some(Status::Request) => Some(0.50),
            Some(Status::Response) => Some(0.75),
            Some(Status::Failure | Status::Redirect | Status::Success) => Some(1.0),
            _ => None,
        };

        // Update components
        self.navigation.update(progress_fraction);
        // @TODO self.content.update();
    }

    // Getters
    pub fn title(&self) -> Option<GString> {
        self.meta.borrow().title.clone()
    }

    pub fn description(&self) -> Option<GString> {
        self.meta.borrow().description.clone()
    }

    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
