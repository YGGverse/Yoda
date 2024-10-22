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

use meta::{Meta, Mime, Status};

use gtk::{
    gio::{Cancellable, SimpleAction, SocketClient, SocketProtocol, TlsCertificateFlags},
    glib::{
        gformat, uuid_string_random, GString, Priority, Regex, RegexCompileFlags, RegexMatchFlags,
        Uri, UriFlags,
    },
    prelude::{
        ActionExt, IOStreamExt, InputStreamExtManual, OutputStreamExtManual, SocketClientExt,
        StaticVariantType, ToVariant,
    },
    Box,
};
use sqlite::Transaction;
use std::{cell::RefCell, sync::Arc};

pub struct Page {
    id: GString,
    // Actions
    action_page_open: Arc<SimpleAction>,
    action_tab_page_navigation_reload: Arc<SimpleAction>,
    action_update: Arc<SimpleAction>,
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
        action_tab_open: Arc<SimpleAction>,
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_update: Arc<SimpleAction>,
    ) -> Arc<Self> {
        // Init local actions
        let action_page_open = Arc::new(SimpleAction::new(
            &uuid_string_random(),
            Some(&String::static_variant_type()),
        ));

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

        // Init globals
        let request_text = self.navigation.request_text();

        // Init shared objects for async access
        let id = self.id.to_variant();
        let navigation = self.navigation.clone();
        let content = self.content.clone();
        let input = self.input.clone();
        let meta = self.meta.clone();
        let action_page_open = self.action_page_open.clone();
        let action_update = self.action_update.clone();

        // Update
        meta.borrow_mut().mime = None;
        meta.borrow_mut().status = Some(Status::Reload);
        meta.borrow_mut().title = Some(gformat!("Loading.."));
        meta.borrow_mut().description = None;

        action_update.activate(Some(&id));

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

                        action_update.activate(Some(&id));

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

                                    action_update.activate(Some(&id));

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

                                                action_update.activate(Some(&id));

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
                                                                    meta.borrow_mut().title = uri.host();

                                                                    action_update.activate(Some(&id));

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
                                                                            // Input expected
                                                                            "10" | "11" => {
                                                                                match parts.get(3) {
                                                                                    Some(placeholder) => {
                                                                                        // Format response
                                                                                        let status = Status::Input;
                                                                                        let title = gformat!("Input expected");
                                                                                        let description = gformat!("{placeholder}");

                                                                                        // Make input form
                                                                                        if "11" == code.as_str() { // sensitive input
                                                                                            input.set_new_sensitive(
                                                                                                action_page_open, uri,
                                                                                                Some(&description),
                                                                                                Some(1024),
                                                                                            );
                                                                                        } else {
                                                                                            input.set_new_response(
                                                                                                action_page_open, uri,
                                                                                                Some(&description),
                                                                                                Some(1024),
                                                                                            );
                                                                                        }

                                                                                        // Update meta
                                                                                        meta.borrow_mut().status = Some(status);
                                                                                        meta.borrow_mut().description = Some(description);
                                                                                        meta.borrow_mut().title = Some(title);

                                                                                        // Update page
                                                                                        action_update.activate(Some(&id));
                                                                                    },
                                                                                    None => todo!(),
                                                                                }
                                                                            },
                                                                            // Success
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
                                                                                                    meta.borrow_mut().title = content.set_text_gemini(&uri, &source);

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
                                                                                                    action_update.activate(Some(&id));
                                                                                                },
                                                                                                None => todo!(),
                                                                                            }
                                                                                        },
                                                                                        "text/plain" => {
                                                                                            meta.borrow_mut().status = Some(Status::Success);
                                                                                            meta.borrow_mut().mime = Some(Mime::TextPlain);

                                                                                            action_update.activate(Some(&id));
                                                                                            todo!()
                                                                                        },
                                                                                        "image/png" | "image/gif" | "image/jpeg" | "image/webp" => {
                                                                                            // Update meta
                                                                                            meta.borrow_mut().status = Some(Status::Success);
                                                                                            meta.borrow_mut().title = Some(gformat!("Picture")); // @TODO
                                                                                            meta.borrow_mut().mime = match mime.as_str() {
                                                                                                "image/png" => Some(Mime::ImagePng),
                                                                                                "image/gif" => Some(Mime::ImageGif),
                                                                                                "image/jpeg" => Some(Mime::ImageJpeg),
                                                                                                "image/webp" => Some(Mime::ImageWebp),
                                                                                                _ => panic!()
                                                                                            };

                                                                                            // Update content
                                                                                            content.set_image(); // @TODO

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
                                                                                            action_update.activate(Some(&id));
                                                                                        },
                                                                                        _ => {
                                                                                            // Define common data
                                                                                            let status = Status::Failure;
                                                                                            let title = gformat!("Oops");
                                                                                            let description = gformat!("Content {mime} not supported");

                                                                                            // Update widget
                                                                                            content.set_status_failure(title.as_str(), description.as_str());

                                                                                            // Update meta
                                                                                            meta.borrow_mut().status = Some(status);
                                                                                            meta.borrow_mut().title = Some(title);
                                                                                            meta.borrow_mut().description = Some(description);

                                                                                            // Update window
                                                                                            action_update.activate(Some(&id));
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

                                                                                action_update.activate(Some(&id));

                                                                                // Select widget
                                                                                match parts.get(3) {
                                                                                    Some(source) => {
                                                                                        let _ = content.set_text_gemini(
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
                                                                                // Define common data
                                                                                let status = Status::Failure;
                                                                                let title = gformat!("Oops");
                                                                                let description = gformat!("Status {code} not supported");

                                                                                // Update widget
                                                                                content.set_status_failure(title.as_str(), description.as_str());

                                                                                // Update meta
                                                                                meta.borrow_mut().status = Some(status);
                                                                                meta.borrow_mut().title = Some(title);
                                                                                meta.borrow_mut().description = Some(description);

                                                                                // Update window
                                                                                action_update.activate(Some(&id));
                                                                            },
                                                                        }
                                                                        None => todo!(),
                                                                    };
                                                                }
                                                                Err(error) => {
                                                                    // Define common data
                                                                    let status = Status::Failure;
                                                                    let title = gformat!("Oops");
                                                                    let description = gformat!("Failed to read buffer data: {:?}", error);

                                                                    // Update widget
                                                                    content.set_status_failure(title.as_str(), description.as_str());

                                                                    // Update meta
                                                                    meta.borrow_mut().status = Some(status);
                                                                    meta.borrow_mut().title = Some(title);
                                                                    meta.borrow_mut().description = Some(description);

                                                                    // Update window
                                                                    action_update.activate(Some(&id));
                                                                }
                                                            }

                                                            // Close connection
                                                            if let Err(error) = connection.close(Some(&cancellable)) {
                                                                todo!("Error closing connection: {}", error.message());
                                                            }
                                                        }
                                                        Err(error) => {
                                                            // Define common data
                                                            let status = Status::Failure;
                                                            let title = gformat!("Oops");
                                                            let description = gformat!("Failed to read response: {}", error.1.message());

                                                            // Update widget
                                                            content.set_status_failure(title.as_str(), description.as_str());

                                                            // Update meta
                                                            meta.borrow_mut().status = Some(status);
                                                            meta.borrow_mut().title = Some(title);
                                                            meta.borrow_mut().description = Some(description);

                                                            // Update window
                                                            action_update.activate(Some(&id));

                                                            // Close connection
                                                            if let Err(error) = connection.close(Some(&cancellable)) {
                                                                todo!("Error closing response connection: {}", error.message());
                                                            }
                                                        }
                                                    },
                                                );
                                            }
                                            Err(error) => {
                                                // Define common data
                                                let status = Status::Failure;
                                                let title = gformat!("Oops");
                                                let description = gformat!("Failed to read request: {}", error.1.message());

                                                // Update widget
                                                content.set_status_failure(title.as_str(), description.as_str());

                                                // Update meta
                                                meta.borrow_mut().status = Some(status);
                                                meta.borrow_mut().title = Some(title);
                                                meta.borrow_mut().description = Some(description);

                                                // Update window
                                                action_update.activate(Some(&id));

                                                // Close connection
                                                if let Err(error) = connection.close(Some(&cancellable)) {
                                                    todo!("Error closing request connection: {}", error.message());
                                                }
                                            }
                                        },
                                    );
                                }
                                Err(error) => {
                                    // Define common data
                                    let status = Status::Failure;
                                    let title = gformat!("Oops");
                                    let description = gformat!("Failed to connect: {}", error.message());

                                    // Update widget
                                    content.set_status_failure(title.as_str(), description.as_str());

                                    // Update meta
                                    meta.borrow_mut().status = Some(status);
                                    meta.borrow_mut().title = Some(title);
                                    meta.borrow_mut().description = Some(description);

                                    // Update window
                                    action_update.activate(Some(&id));
                                }
                            },
                        );
                    }
                    /* @TODO
                    "nex" => {}
                    */
                    scheme => {
                        // Define common data
                        let status = Status::Failure;
                        let title = gformat!("Oops");
                        let description = gformat!("Protocol `{scheme}` not supported");

                        // Update widget
                        content.set_status_failure(title.as_str(), description.as_str());

                        // Update meta
                        meta.borrow_mut().status = Some(status);
                        meta.borrow_mut().title = Some(title);
                        meta.borrow_mut().description = Some(description);

                        // Update window
                        action_update.activate(Some(&id));
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
        };
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
            Some(Status::Prepare | Status::Reload) => Some(0.0),
            Some(Status::Connect) => Some(0.25),
            Some(Status::Request) => Some(0.50),
            Some(Status::Response) => Some(0.75),
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
}
