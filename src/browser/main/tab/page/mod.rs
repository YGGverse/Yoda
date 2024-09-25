mod content;
mod navigation;

use content::Content;
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

// Extras
enum Mime {
    Undefined,
    TextGemini,
    TextPlain,
}

struct Meta {
    title: GString,
    description: GString,
    mime: Mime,
    progress_fraction: f32,
}

// Main
pub struct Page {
    // GTK
    widget: Box,
    // Components
    navigation: Navigation,
    content: Arc<Content>,
    // Extras
    meta: RefCell<Meta>,
}

impl Page {
    // Construct
    pub fn new(name: GString) -> Arc<Page> {
        // Init components
        let content = Content::new();
        let navigation = Navigation::new();

        // Init widget
        let widget = Box::builder()
            .orientation(Orientation::Vertical)
            .name(name)
            .build();

        widget.append(navigation.widget());
        widget.append(content.widget());

        // Init meta
        let meta = RefCell::new(Meta {
            title: GString::new(),
            description: GString::new(),
            mime: Mime::Undefined,
            progress_fraction: 0.0,
        });

        // Result
        Arc::new(Self {
            widget,
            content,
            navigation,
            meta,
        })
    }

    // Actions
    pub fn reload(&self) {
        // Init globals
        let request_text = self.navigation.request_text();

        // Update
        self.meta.borrow_mut().title = GString::from("Reload");
        self.meta.borrow_mut().description = request_text.clone();
        self.meta.borrow_mut().mime = Mime::Undefined;
        self.meta.borrow_mut().progress_fraction = 0.0;

        let _ = self.widget.activate_action("win.update", None);

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
                        self.meta.borrow_mut().title = GString::from("Connect");
                        //self.meta.borrow_mut().description = uri.host();
                        self.meta.borrow_mut().progress_fraction = 0.25;

                        let _ = self.widget.activate_action("win.update", None);

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
                                    //self.meta.borrow_mut().title = GString::from("Request");
                                    //self.meta.borrow_mut().progress_fraction = 0.50;

                                    //let _ = self.widget.activate_action("win.update", None);

                                    // Send request
                                    connection.output_stream().write_all_async(
                                        "gemini://geminiprotocol.net:1965/\r\n", // @TODO
                                        Priority::DEFAULT,
                                        Some(&Cancellable::new()),
                                        move |result| match result {
                                            Ok(_) => {
                                                // Update
                                                //self.meta.borrow_mut().title = GString::from("Response");
                                                //self.meta.borrow_mut().progress_fraction = 0.75;

                                                //let _ = self.widget.activate_action("win.update", None);

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
                                                                    //self.meta.borrow_mut().title = GString::from("Done"); // @TODO
                                                                    //self.meta.borrow_mut().progress_fraction = 1.0;

                                                                    //let _ = self.widget.activate_action("win.update", None);

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
