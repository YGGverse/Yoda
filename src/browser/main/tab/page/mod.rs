mod content;
mod meta;
mod navigation;

use content::Content;
use meta::{Meta, Mime};
use navigation::Navigation;

use gtk::{
    gio::{Cancellable, SocketClient, SocketProtocol, TlsCertificateFlags},
    glib::{GString, Priority, Regex, RegexCompileFlags, RegexMatchFlags, Uri, UriFlags},
    prelude::{
        BoxExt, IOStreamExt, InputStreamExtManual, OutputStreamExtManual, SocketClientExt,
        WidgetExt,
    },
    Box, Orientation,
};
use std::{cell::RefCell, sync::Arc};

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
    pub fn new(name: GString) -> Page {
        // Init components
        let content = Arc::new(Content::new());
        let navigation = Arc::new(Navigation::new());

        // Init widget
        let widget = Box::builder()
            .orientation(Orientation::Vertical)
            .name(name)
            .build();

        widget.append(navigation.widget());
        widget.append(content.widget());

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
        let meta = self.meta.clone();
        let widget = self.widget.clone();

        // Update
        meta.borrow_mut().title = GString::from("Reload");
        meta.borrow_mut().description = request_text.clone();
        meta.borrow_mut().mime = Mime::Undefined;
        meta.borrow_mut().progress_fraction = 0.0;

        let _ = widget.activate_action("win.update", None);

        /*let _uri = */
        match Uri::parse(&request_text, UriFlags::NONE) {
            Ok(uri) => {
                // Route request by scheme
                match uri.scheme().as_str() {
                    "file" => {
                        todo!()
                    }
                    "gemini" => {
                        // Update
                        meta.borrow_mut().title = GString::from("Connect");
                        meta.borrow_mut().description = match uri.host() {
                            Some(host) => host,
                            None => panic!(),
                        };
                        meta.borrow_mut().progress_fraction = 0.25;

                        let _ = widget.activate_action("win.update", None);

                        // Create new connection
                        let client = SocketClient::new();

                        client.set_timeout(10);
                        client.set_tls(true);
                        client.set_tls_validation_flags(TlsCertificateFlags::INSECURE);
                        client.set_protocol(SocketProtocol::Tcp);

                        client.connect_to_uri_async(
                            "gemini://geminiprotocol.net:1965/", // @TODO &uri.to_str(),
                            1965,
                            Some(&Cancellable::new()),
                            move |result| match result {
                                Ok(connection) => {
                                    // Update
                                    meta.borrow_mut().title = GString::from("Request");
                                    meta.borrow_mut().progress_fraction = 0.50;

                                    let _ = widget.activate_action("win.update", None);

                                    // Send request
                                    connection.output_stream().write_all_async(
                                        "gemini://geminiprotocol.net:1965/\r\n", // @TODO
                                        Priority::DEFAULT,
                                        Some(&Cancellable::new()),
                                        move |result| match result {
                                            Ok(_) => {
                                                // Update
                                                meta.borrow_mut().title = GString::from("Response");
                                                meta.borrow_mut().progress_fraction = 0.75;

                                                let _ = widget.activate_action("win.update", None);

                                                // Read response
                                                connection.input_stream().read_all_async(
                                                    vec![0; 0xfffff], // 1Mb @TODO
                                                    Priority::DEFAULT,
                                                    Some(&Cancellable::new()),
                                                    move |result| match result {
                                                        Ok(response) => {
                                                            match GString::from_utf8_until_nul(
                                                                response.0,
                                                            ) {
                                                                Ok(data) => {
                                                                    // Update
                                                                    meta.borrow_mut().title =
                                                                        GString::from("Done"); // @TODO
                                                                    meta.borrow_mut()
                                                                        .progress_fraction = 1.0;

                                                                    let _ = widget.activate_action(
                                                                        "win.update",
                                                                        None,
                                                                    );

                                                                    // Detect page meta
                                                                    let meta = Regex::split_simple(
                                                                        r"^(\d+)?\s([\w]+\/[\w]+)?",
                                                                        &data,
                                                                        RegexCompileFlags::DEFAULT,
                                                                        RegexMatchFlags::DEFAULT,
                                                                    );

                                                                    //println!("{:?}", meta);
                                                                    //println!("Result: {}", data);
                                                                }
                                                                Err(e) => {
                                                                    eprintln!(
                                                                        "Failed to read buffer: {e}"
                                                                    )
                                                                }
                                                            }

                                                            // @TODO connection.close(cancellable);
                                                        }
                                                        Err(e) => {
                                                            eprintln!(
                                                                "Failed to read response: {:?}",
                                                                e
                                                            );
                                                        }
                                                    },
                                                );
                                            }
                                            Err(e) => {
                                                eprintln!("Failed to write request: {:?}", e);
                                            }
                                        },
                                    );
                                }
                                Err(e) => {
                                    eprintln!("Failed to connect: {e}"); // @TODO
                                }
                            },
                        );
                    }
                    "nex" => {
                        todo!()
                    }
                    scheme => {
                        println!("Protocol {scheme} not supported");
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
                    let request_text = GString::from(format!("gemini://{request_text}"));
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
                        &GString::from(format!(
                            "gemini://tlgs.one/search?{}",
                            Uri::escape_string(&request_text, None, false)
                        )),
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
    pub fn title(&self) -> GString {
        self.meta.borrow().title.clone()
    }

    pub fn description(&self) -> GString {
        self.meta.borrow().description.clone()
    }

    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
