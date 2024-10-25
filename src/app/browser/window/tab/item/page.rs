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
    gio::SimpleAction,
    glib::{
        gformat, uuid_string_random, GString, Regex, RegexCompileFlags, RegexMatchFlags, Uri,
        UriFlags,
    },
    prelude::{ActionExt, StaticVariantType, ToVariant},
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
                        // Define local NS
                        use gemini::client::{
                            response::header::{Mime as ResponseMime, Status as ResponseStatus},
                            simple_socket_request_async,
                        };

                        // Update page status
                        meta.borrow_mut().status = Some(Status::Connecting);
                        action_update.activate(Some(&id));

                        // Begin request
                        simple_socket_request_async(uri.clone(), move |result| match result {
                            Ok(response) => {
                                // Format response
                                meta.borrow_mut().status = Some(Status::Connected);
                                meta.borrow_mut().title = uri.host();
                                action_update.activate(Some(&id));

                                match response.header().status() {
                                    // 10 | 11
                                    Some(ResponseStatus::Input)
                                    | Some(ResponseStatus::SensitiveInput) => {
                                        // Format response
                                        let status = Status::Input;
                                        let title = gformat!("Input expected");
                                        let description = match response.header().meta() {
                                            Some(meta) => match meta.to_gstring() {
                                                Ok(value) => value,
                                                Err(_) => title.clone(),
                                            },
                                            None => title.clone(),
                                        };

                                        // Make input form
                                        match response.header().status() {
                                            Some(ResponseStatus::SensitiveInput) => input
                                                .set_new_sensitive(
                                                    action_page_open,
                                                    uri,
                                                    Some(&description),
                                                    Some(1024),
                                                ),
                                            _ => input.set_new_response(
                                                action_page_open,
                                                uri,
                                                Some(&description),
                                                Some(1024),
                                            ),
                                        }

                                        // Update meta
                                        meta.borrow_mut().status = Some(status);
                                        meta.borrow_mut().description = Some(description);
                                        meta.borrow_mut().title = Some(title);

                                        // Update page
                                        action_update.activate(Some(&id));
                                    }
                                    // 20
                                    Some(ResponseStatus::Success) => match response.header().mime()
                                    {
                                        Some(ResponseMime::TextGemini) => {
                                            // Update data
                                            match response.body().to_gstring() {
                                                Ok(source) => {
                                                    meta.borrow_mut().status =
                                                        Some(Status::Success);

                                                    // This content type may return parsed title
                                                    meta.borrow_mut().title =
                                                        content.set_text_gemini(&uri, &source);

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
                                                }
                                                Err(_) => todo!(),
                                            }
                                        }
                                        Some(ResponseMime::TextPlain) => {
                                            meta.borrow_mut().status = Some(Status::Success);

                                            action_update.activate(Some(&id));
                                            todo!()
                                        }
                                        Some(ResponseMime::ImagePng)
                                        | Some(ResponseMime::ImageGif)
                                        | Some(ResponseMime::ImageJpeg)
                                        | Some(ResponseMime::ImageWebp) => {
                                            // Update meta
                                            meta.borrow_mut().status = Some(Status::Success);
                                            meta.borrow_mut().title = Some(gformat!("Picture")); // @TODO

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
                                        }
                                        _ => {
                                            // Define common data
                                            let status = Status::Failure;
                                            let title = gformat!("Oops");
                                            let description =
                                                gformat!("Content type not supported");

                                            // Update widget
                                            content.set_status_failure(
                                                title.as_str(),
                                                description.as_str(),
                                            );

                                            // Update meta
                                            meta.borrow_mut().status = Some(status);
                                            meta.borrow_mut().title = Some(title);
                                            meta.borrow_mut().description = Some(description);

                                            // Update window
                                            action_update.activate(Some(&id));
                                        }
                                    },
                                    // 31
                                    Some(ResponseStatus::Redirect) => {
                                        // Update meta
                                        meta.borrow_mut().status = Some(Status::Redirect);
                                        meta.borrow_mut().title = Some(gformat!("Redirect"));

                                        action_update.activate(Some(&id));

                                        // Select widget
                                        match response.header().meta() {
                                            Some(meta) => {
                                                let _ = content.set_text_gemini(
                                                        &uri,
                                                        // @TODO use template file
                                                        &gformat!(
                                                            "# Redirect\n\nAuto-follow disabled, click on link below to continue\n\n=> {}",
                                                            match meta.to_gstring() {
                                                                Ok(url) => url,
                                                                Err(_) => todo!()
                                                            }
                                                        )
                                                    );
                                            }
                                            None => todo!(),
                                        }
                                    }
                                    // @TODO
                                    None => {
                                        // Define common data
                                        let status = Status::Failure;
                                        let title = gformat!("Oops");
                                        let description = gformat!("Status code not supported");

                                        // Update widget
                                        content.set_status_failure(
                                            title.as_str(),
                                            description.as_str(),
                                        );

                                        // Update meta
                                        meta.borrow_mut().status = Some(status);
                                        meta.borrow_mut().title = Some(title);
                                        meta.borrow_mut().description = Some(description);

                                        // Update window
                                        action_update.activate(Some(&id));
                                    }
                                };
                            }
                            Err(reason) => {
                                // Define common data
                                let status = Status::Failure;
                                let title = gformat!("Oops");
                                let description = match reason {
                                    gemini::client::Error::Connection => {
                                        gformat!("Failed to connect")
                                    }
                                    gemini::client::Error::Request => {
                                        gformat!("Failed to send request")
                                    }
                                    gemini::client::Error::Response => {
                                        gformat!("Failed to read response")
                                    }
                                    gemini::client::Error::Close => {
                                        gformat!("Failed to close connection")
                                    }
                                }; // @TODO explain

                                // Update widget
                                content.set_status_failure(title.as_str(), description.as_str());

                                // Update meta
                                meta.borrow_mut().status = Some(status);
                                meta.borrow_mut().title = Some(title);
                                meta.borrow_mut().description = Some(description);

                                // Update window
                                action_update.activate(Some(&id));
                            }
                        });
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
            Some(Status::Reload) => Some(0.0),
            Some(Status::Connecting) => Some(0.25),
            Some(Status::Connected) => Some(0.50),
            // Some(Status::Response) => Some(0.75),
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
