mod content;
mod meta;
mod navigation;

use content::Content;
use meta::{Meta, Mime};
use navigation::Navigation;

use gtk::{
    gio::{
        ActionEntry, Cancellable, SimpleActionGroup, SocketClient, SocketProtocol,
        TlsCertificateFlags,
    },
    glib::{gformat, GString, Priority, Regex, RegexCompileFlags, RegexMatchFlags, Uri, UriFlags},
    prelude::{
        ActionMapExtManual, BoxExt, IOStreamExt, InputStreamExtManual, OutputStreamExtManual,
        SocketClientExt, StaticVariantType, WidgetExt,
    },
    Box, Orientation,
};
use std::{cell::RefCell, path::Path, sync::Arc};

pub struct Page {
    // GTK
    widget: Box,
    // Components
    navigation: Arc<Navigation>,
    content: Arc<Content>,
    // Extras
    meta: Arc<RefCell<Meta>>,
}

impl Page {
    // Construct
    pub fn new(name: GString, navigation_request_text: Option<GString>) -> Page {
        // Init components
        let content = Arc::new(Content::new());
        let navigation = Arc::new(Navigation::new(navigation_request_text));

        // Init widget
        let widget = Box::builder()
            .orientation(Orientation::Vertical)
            .name(name)
            .build();

        widget.append(navigation.widget());
        widget.append(content.widget());

        // Init actions @TODO move actions init outside
        let action_open = ActionEntry::builder("open")
            .parameter_type(Some(&String::static_variant_type()))
            .activate({
                let navigation = navigation.clone();
                move |_, _, request| {
                    let uri = request
                        .expect("Parameter required for `page.open` action")
                        .get::<String>()
                        .expect("Parameter does not match `String`");

                    navigation.set_request_text(
                        &GString::from(uri),
                        true, // activate (page reload)
                    );
                }
            })
            .build();

        // Init action group
        let actions = SimpleActionGroup::new();
        actions.add_action_entries([action_open]);
        widget.insert_action_group("page", Some(&actions));

        // Init async mutable Meta object
        let meta = Arc::new(RefCell::new(Meta::new()));

        // Result
        Self {
            widget,
            content,
            navigation,
            meta,
        }
    }

    // Actions
    pub fn reload(&self) {
        // Init globals
        let request_text = self.navigation.request_text();

        // Init shared objects for async access
        let content = self.content.clone();
        let meta = self.meta.clone();
        let widget = self.widget.clone();

        // Update
        meta.borrow_mut().mime = None;
        meta.borrow_mut().title = Some(gformat!("Loading.."));
        meta.borrow_mut().description = None;
        meta.borrow_mut().progress_fraction = 0.0;

        widget
            .activate_action("win.update", None)
            .expect("Action `win.update` not found");

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
                            None => panic!(),
                        };

                        // Update
                        meta.borrow_mut().progress_fraction = 0.25;
                        meta.borrow_mut().description = Some(gformat!("Connect {host}.."));

                        widget
                            .activate_action("win.update", None)
                            .expect("Action `win.update` not found");

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
                                    meta.borrow_mut().progress_fraction = 0.50;
                                    meta.borrow_mut().description = Some(gformat!("Connected to {host}.."));

                                    widget.activate_action("win.update", None).expect("Action `win.update` not found");

                                    // Send request
                                    connection.output_stream().write_all_async(
                                        gformat!("{}\r\n", &uri.to_str()),
                                        Priority::DEFAULT,
                                        Some(&cancellable.clone()),
                                        move |result| match result {
                                            Ok(_) => {
                                                // Update
                                                meta.borrow_mut().progress_fraction = 0.75;
                                                meta.borrow_mut().description = Some(gformat!("Request data from {host}.."));

                                                widget.activate_action("win.update", None).expect("Action `win.update` not found");

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
                                                                    meta.borrow_mut().progress_fraction = 1.0;
                                                                    meta.borrow_mut().description = Some(host);
                                                                    meta.borrow_mut().title = Some(uri.path());

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
                                                                                            // Select widget
                                                                                            match parts.get(4) {
                                                                                                Some(source) => content.reset(content::Mime::TextGemini, &uri, &source),
                                                                                                None => todo!(),
                                                                                            }
                                                                                        },
                                                                                        "text/plain" => {
                                                                                            meta.borrow_mut().mime = Some(Mime::TextPlain);
                                                                                            todo!()
                                                                                        },
                                                                                        _ => {
                                                                                            meta.borrow_mut().title = Some(gformat!("Oops"));
                                                                                            meta.borrow_mut().description = Some(gformat!("Content {mime} not supported"));
                                                                                        },
                                                                                    }
                                                                                    None => todo!(),
                                                                                };
                                                                                // @TODO
                                                                            },
                                                                            _ => {
                                                                                meta.borrow_mut().title = Some(gformat!("Oops"));
                                                                                meta.borrow_mut().description = Some(gformat!("Status {code} not supported"));
                                                                            },
                                                                        }
                                                                        None => todo!(),
                                                                    };

                                                                    // Update
                                                                    let _ = widget.activate_action(
                                                                        "win.update",
                                                                        None,
                                                                    );
                                                                }
                                                                Err(e) => {
                                                                    meta.borrow_mut().title = Some(gformat!("Oops"));
                                                                    meta.borrow_mut().description = Some(gformat!("Failed to read buffer data: {e}"));
                                                                    meta.borrow_mut().progress_fraction = 1.0;

                                                                    let _ = widget.activate_action(
                                                                        "win.update",
                                                                        None,
                                                                    );
                                                                }
                                                            }

                                                            // Close connection
                                                            if let Err(e) = connection.close(Some(&cancellable)) {
                                                                panic!("Error closing connection: {:?}", e);
                                                            }
                                                        }
                                                        Err(e) => {
                                                            // Update
                                                            meta.borrow_mut().title = Some(gformat!("Oops"));
                                                            meta.borrow_mut().description = Some(gformat!("Failed to read response: {:?}", e));
                                                            meta.borrow_mut().progress_fraction = 1.0;

                                                            let _ = widget.activate_action(
                                                                "win.update",
                                                                None,
                                                            );

                                                            // Close connection
                                                            if let Err(e) = connection.close(Some(&cancellable)) {
                                                                panic!("Error closing response connection: {:?}", e);
                                                            }
                                                        }
                                                    },
                                                );
                                            }
                                            Err(e) => {
                                                // Update
                                                meta.borrow_mut().title = Some(gformat!("Oops"));
                                                meta.borrow_mut().description = Some(gformat!("Failed to read request: {:?}", e));
                                                meta.borrow_mut().progress_fraction = 1.0;

                                                let _ = widget.activate_action(
                                                    "win.update",
                                                    None,
                                                );

                                                // Close connection
                                                if let Err(e) = connection.close(Some(&cancellable)) {
                                                    panic!("Error closing request connection: {:?}", e);
                                                }
                                            }
                                        },
                                    );
                                }
                                Err(e) => {
                                    // Update
                                    meta.borrow_mut().title = Some(gformat!("Oops"));
                                    meta.borrow_mut().description = Some(gformat!("Failed to connect: {:?}", e));
                                    meta.borrow_mut().progress_fraction = 1.0;

                                    let _ = widget.activate_action(
                                        "win.update",
                                        None,
                                    );
                                }
                            },
                        );
                    }
                    /* @TODO
                    "nex" => {}
                    */
                    scheme => {
                        // Update
                        meta.borrow_mut().title = Some(gformat!("Oops"));
                        meta.borrow_mut().description =
                            Some(gformat!("Protocol {scheme} not supported"));
                        meta.borrow_mut().progress_fraction = 1.0;

                        widget
                            .activate_action("win.update", None)
                            .expect("Action `win.update` not found");
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
                            self.navigation.set_request_text(
                                &request_text,
                                true, // activate (page reload)
                            );
                        }
                        Err(_) => {
                            // @TODO any action here?
                        }
                    }
                } else {
                    // Plain text given, make search request to default provider
                    self.navigation.set_request_text(
                        &gformat!(
                            "gemini://tlgs.one/search?{}",
                            Uri::escape_string(&request_text, None, false)
                        ),
                        true, // activate (page reload)
                    );
                }
            }
        };
    }

    pub fn update(&self) {
        self.navigation.update();
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
