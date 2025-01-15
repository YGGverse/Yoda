mod client;
mod content;
mod database;
mod error;
mod input;
mod mode;
mod navigation;
mod search;
mod status;
mod widget;

use client::Client;
use content::Content;
use error::Error;
use input::Input;
use mode::Mode;
use navigation::Navigation;
use search::Search;
use status::Status;
use widget::Widget;

use super::{Action as TabAction, BrowserAction, Profile, WindowAction};

use gtk::{
    gdk::Texture,
    gdk_pixbuf::Pixbuf,
    gio::SocketClientEvent,
    glib::{gformat, GString, Priority, Uri, UriFlags, UriHideFlags},
    prelude::{EditableExt, FileExt, SocketClientExt},
};
use sqlite::Transaction;
use std::{cell::RefCell, rc::Rc, time::Duration};

pub struct Page {
    id: Rc<GString>,
    profile: Rc<Profile>,
    status: Rc<RefCell<Status>>,
    title: Rc<RefCell<GString>>,
    // Actions
    browser_action: Rc<BrowserAction>,
    tab_action: Rc<TabAction>,
    window_action: Rc<WindowAction>,
    // Components
    pub client: Rc<Client>,
    pub content: Rc<Content>,
    pub search: Rc<Search>,
    pub input: Rc<Input>,
    pub navigation: Rc<Navigation>,
    pub widget: Rc<Widget>,
}

impl Page {
    // Constructors

    pub fn build(
        id: &Rc<GString>,
        profile: &Rc<Profile>,
        (browser_action, window_action, tab_action): (
            &Rc<BrowserAction>,
            &Rc<WindowAction>,
            &Rc<TabAction>,
        ),
    ) -> Self {
        // Init components
        let content = Rc::new(Content::build((window_action, tab_action)));

        let search = Rc::new(Search::new());

        let navigation = Rc::new(Navigation::build(
            profile,
            (browser_action, window_action, tab_action),
        ));

        let input = Rc::new(Input::new());

        let widget = Rc::new(Widget::build(
            id,
            &navigation.widget.g_box,
            &content.g_box,
            &search.g_box,
            &input.widget.clamp,
        ));

        //let meta = Rc::new(Meta::new(Status::New, gformat!("New page")));

        // Done
        Self {
            id: id.clone(),
            profile: profile.clone(),
            status: Rc::new(RefCell::new(Status::New)),
            title: Rc::new(RefCell::new(gformat!("New page"))),
            // Actions
            browser_action: browser_action.clone(),
            tab_action: tab_action.clone(),
            window_action: window_action.clone(),
            // Components
            client: Rc::new(Client::new()),
            content,
            search,
            input,
            navigation,
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
            .toggle(self.navigation.request.widget.entry.text().as_str())
        {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::Bookmark), // @TODO
        };
        self.update();
        result
    }

    /// Request `Escape` action for child components
    pub fn escape(&self) {
        self.search.hide()
    }

    /// Toggle `Find` widget
    pub fn find(&self) {
        self.search.show()
    }

    /// Navigate home URL (parsed from current navigation entry)
    /// * this method create new history record in memory as defined in `action_page_open` action
    pub fn home(&self) {
        if let Some(url) = self.navigation.home.url() {
            // Update with history record
            self.tab_action.load.activate(Some(&url), true);
        }
    }

    /// Navigate back in history
    /// * this method does not create new history record in memory
    pub fn history_back(&self) {
        if let Some(request) = self.navigation.history.back(true) {
            // Update navigation entry
            self.navigation.request.widget.entry.set_text(&request);

            // Load page (without history record)
            self.load(false);
        }
    }

    /// Navigate forward in history
    /// * this method does not create new history record in memory
    pub fn history_forward(&self) {
        if let Some(request) = self.navigation.history.forward(true) {
            // Update navigation entry
            self.navigation.request.widget.entry.set_text(&request);

            // Load page (without history record)
            self.load(false);
        }
    }

    /// Main loader for different protocols, that routed by scheme
    /// * every protocol has it own (children) method implementation
    pub fn load(&self, is_history: bool) {
        // Move focus out from navigation entry
        self.browser_action
            .escape
            .activate_stateful_once(Some(self.id.as_str().into()));

        // Initially disable find action
        self.window_action.find.simple_action.set_enabled(false);

        // Reset widgets
        self.search.unset();
        self.input.unset();

        // Try redirect request
        let request = if let Some(redirect) = self.client.redirect.last() {
            // Gemini protocol may provide background (temporarily) redirects
            if redirect.is_foreground {
                self.navigation
                    .request
                    .widget
                    .entry
                    .set_text(&redirect.request.to_string());
            }

            // Return value from redirection holder
            Mode::from(&redirect.request.to_str(), None /*redirect.referrer*/) // @TODO
        } else {
            // Reset redirect counter as request value taken from user input
            self.client.redirect.clear();

            // Return value from navigation entry
            Mode::from(&self.navigation.request.widget.entry.text(), None)
        };

        // Update
        self.status.replace(Status::Reload);
        self.title.replace(gformat!("Loading.."));
        self.browser_action.update.activate(Some(&self.id));

        // Route by `Mode`
        match request {
            Mode::Default(ref uri) | Mode::Download(ref uri) | Mode::Source(ref uri) => {
                // Route by scheme
                match uri.scheme().as_str() {
                    "file" => todo!(),
                    "gemini" => {
                        let (uri, is_download, is_source) = match request {
                            Mode::Default(uri) => (uri, false, false),
                            Mode::Download(uri) => (uri, true, false),
                            Mode::Source(uri) => (uri, false, true),
                            _ => panic!(),
                        };
                        self.load_gemini(uri, is_download, is_source, is_history)
                    }
                    "titan" => {
                        // Toggle input form
                        // @TODO self.input.set_new_titan(|data|{});

                        // Update meta
                        self.status.replace(Status::Input);
                        self.title.replace(gformat!("Titan input"));

                        // Update page
                        self.browser_action.update.activate(Some(&self.id));
                    }
                    scheme => {
                        // Add history record
                        if is_history {
                            snap_history(&self.profile, &self.navigation, Some(uri));
                        }

                        // Update widget
                        let status = self.content.to_status_failure();
                        status.set_description(Some(&format!("Scheme `{scheme}` not supported")));

                        // Update meta
                        self.status.replace(Status::Failure);
                        self.title.replace(status.title());

                        // Update window
                        self.browser_action.update.activate(Some(&self.id));
                    }
                }
            }
            Mode::Search(query) => {
                // try autocomplete scheme and request it on successful resolve
                // otherwise make search request @TODO optional search provider
                self.navigation
                    .request
                    .to_gemini_async(500, Some(&self.client.cancellable()), {
                        let tab_action = self.tab_action.clone();
                        move |result| {
                            tab_action.load.activate(
                                Some(&match result {
                                    Some(url) => url,
                                    None => gformat!(
                                        "gemini://tlgs.one/search?{}",
                                        Uri::escape_string(&query, None, false)
                                    ),
                                }),
                                true,
                            )
                        }
                    });
            }
        };
    }

    /// Update `Self` witch children components
    pub fn update(&self) {
        // Update components
        self.navigation.update(self.to_progress_fraction());
        // @TODO self.content.update();
    }

    /// Cleanup `Self` session
    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_id: i64,
    ) -> Result<(), String> {
        match database::select(transaction, app_browser_window_tab_item_id) {
            Ok(records) => {
                for record in records {
                    match database::delete(transaction, record.id) {
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

    /// Restore `Self` session from database
    pub fn restore(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_id: i64,
    ) -> Result<(), String> {
        // Update status
        self.status.replace(Status::SessionRestore);

        // Begin page restore
        match database::select(transaction, app_browser_window_tab_item_id) {
            Ok(records) => {
                for record in records {
                    // Restore self by last record
                    self.title.replace(record.title.into());
                    // Delegate restore action to the item childs
                    self.navigation.restore(transaction, &record.id)?;
                    // Make initial page history snap using `navigation` values restored
                    // * just to have back/forward navigation ability
                    snap_history(&self.profile, &self.navigation, None);
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        // Update status
        self.status.replace(Status::SessionRestored);

        Ok(())
    }

    /// Save `Self` session to database
    pub fn save(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_id: i64,
    ) -> Result<(), String> {
        match database::insert(
            transaction,
            app_browser_window_tab_item_id,
            self.title.borrow().as_str(),
        ) {
            Ok(_) => {
                let id = database::last_insert_id(transaction);

                // Delegate save action to childs
                self.navigation.save(transaction, &id)?;
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    // Getters

    /// Get `title` copy from `Self`
    pub fn title(&self) -> GString {
        self.title.borrow().clone()
    }

    /// Convert `Self` to `progress-fraction` presentation
    /// * see also: [Entry](https://docs.gtk.org/gtk4/property.Entry.progress-fraction.html)
    pub fn to_progress_fraction(&self) -> Option<f64> {
        // Interpret status to progress fraction
        match *self.status.borrow() {
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

    /// Get `Self` loading status
    pub fn is_loading(&self) -> bool {
        match self.to_progress_fraction() {
            Some(progress_fraction) => progress_fraction < 1.0,
            None => false,
        }
    }

    // Private helpers

    // @TODO move outside
    fn load_gemini(&self, uri: Uri, is_download: bool, is_source: bool, is_history: bool) {
        // Init local namespace
        use gemini::client::connection::{response, Request};

        // Init shared clones
        let browser_action = self.browser_action.clone();
        let cancellable = self.client.cancellable();
        let content = self.content.clone();
        let id = self.id.clone();
        let input = self.input.clone();
        let title = self.title.clone();
        let status = self.status.clone();
        let navigation = self.navigation.clone();
        let profile = self.profile.clone();
        let search = self.search.clone();
        let tab_action = self.tab_action.clone();
        let window_action = self.window_action.clone();
        let redirect = self.client.redirect.clone();

        // Listen for connection status updates
        self.client.gemini.socket.connect_event({
            let id = id.clone();
            let status = status.clone();
            let update = browser_action.update.clone();
            move |_, event, _, _| {
                status.replace(match event {
                    SocketClientEvent::Resolving => Status::Resolving,
                    SocketClientEvent::Resolved => Status::Resolved,
                    SocketClientEvent::Connecting => Status::Connecting,
                    SocketClientEvent::Connected => Status::Connected,
                    SocketClientEvent::ProxyNegotiating => Status::ProxyNegotiating,
                    SocketClientEvent::ProxyNegotiated => Status::ProxyNegotiated,
                    // TlsHandshaking have effect only for guest connections!
                    SocketClientEvent::TlsHandshaking => Status::TlsHandshaking,
                    SocketClientEvent::TlsHandshaked => Status::TlsHandshaked,
                    SocketClientEvent::Complete => Status::Complete,
                    _ => todo!(), // notice on API change
                });
                update.activate(Some(&id));
            }
        });

        // Begin new socket request
        self.client.gemini.request_async(
            Request::gemini(uri.clone()),
            Priority::DEFAULT,
            cancellable.clone(),
            // Search for user certificate match request
            match self.profile.identity.gemini.match_scope(&uri.to_string()) {
                Some(identity) => match identity.to_tls_certificate() {
                    Ok(certificate) => Some(certificate),
                    Err(e) => todo!("{e}"),
                },
                None => None,
            },
            move |result| match result {
                Ok(response) => {
                    // Route by status
                    match response.meta.status {
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#input-expected
                        response::meta::Status::Input |
                        response::meta::Status::SensitiveInput => {
                            // Format response
                            let header = match response.meta.data {
                                Some(data) => data.value,
                                None => gformat!("Input expected"),
                            };

                            // Toggle input form variant
                            match response.meta.status {
                                response::meta::Status::SensitiveInput =>
                                    input.set_new_sensitive(
                                        tab_action.clone(),
                                        uri.clone(),
                                        Some(&header),
                                        Some(1024),
                                    ),
                                _ =>
                                    input.set_new_response(
                                        tab_action.clone(),
                                        uri.clone(),
                                        Some(&header),
                                        Some(1024),
                                    ),
                            }

                            // Update meta
                            status.replace(Status::Input);
                            title.replace(header);

                            // Update page
                            browser_action.update.activate(Some(&id));
                        },
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-20
                        response::meta::Status::Success => {
                            if is_history {
                                snap_history(&profile, &navigation, Some(&uri));
                            }
                            if is_download {
                                // Init download widget
                                let status_page = content.to_status_download(
                                    &uri_to_title(&uri), // grab default filename
                                    &cancellable,
                                    {
                                        let cancellable = cancellable.clone();
                                        move |file, action| {
                                            match file.replace(
                                                None,
                                                false,
                                                gtk::gio::FileCreateFlags::NONE,
                                                Some(&cancellable)
                                            ) {
                                                Ok(file_output_stream) => {
                                                    gemini::gio::file_output_stream::move_all_from_stream_async(
                                                        response.connection.stream(),
                                                        file_output_stream,
                                                        cancellable.clone(),
                                                        Priority::DEFAULT,
                                                        (
                                                            0x100000, // 1M bytes per chunk
                                                            None,     // unlimited
                                                            0         // initial totals
                                                        ),
                                                        (
                                                            // on chunk
                                                            {
                                                                let action = action.clone();
                                                                move |_, total| action.update.activate(
                                                                    &format!(
                                                                        "Received {}...",
                                                                        crate::tool::format_bytes(total)
                                                                    )
                                                                )
                                                            },
                                                            // on complete
                                                            {
                                                                let action = action.clone();
                                                                move |result| match result {
                                                                    Ok((_, total)) => action.complete.activate(
                                                                        &format!("Saved to {} ({total} bytes total)", file.parse_name())
                                                                    ),
                                                                    Err(e) => action.cancel.activate(&e.to_string())
                                                                }
                                                            }
                                                        )
                                                    );
                                                },
                                                Err(e) => action.cancel.activate(&e.to_string())
                                            }
                                        }
                                    }
                                );

                                // Update meta
                                status.replace(Status::Success);
                                title.replace(status_page.title());

                                // Update window
                                browser_action.update.activate(Some(&id));
                            } else { // browse
                                match response.meta.mime.unwrap().value.to_lowercase().as_str() {
                                    "text/gemini" => {
                                        // Read entire input stream to buffer
                                        response::data::Text::from_stream_async(
                                            response.connection.stream(),
                                            Priority::DEFAULT,
                                            cancellable.clone(),
                                            {
                                                let browser_action = browser_action.clone();
                                                let content = content.clone();
                                                let id = id.clone();
                                                let search = search.clone();
                                                let status = status.clone();
                                                let title = title.clone();
                                                let uri = uri.clone();
                                                let window_action = window_action.clone();
                                                move |result| {
                                                    match result {
                                                        Ok(buffer) => {
                                                            // Set children component,
                                                            // extract title from meta parsed
                                                            let text_widget = if is_source {
                                                                content.to_text_source(
                                                                    &buffer.data
                                                                )
                                                            } else {
                                                                content.to_text_gemini(
                                                                    &uri,
                                                                    &buffer.data
                                                                )
                                                            };

                                                            // Connect `TextView` widget, update `search` model
                                                            search.set(Some(text_widget.text_view));

                                                            // Update page meta
                                                            status.replace(Status::Success);
                                                            title.replace(match text_widget.meta.title {
                                                                Some(meta_title) => meta_title.into(), // @TODO
                                                                None => uri_to_title(&uri)
                                                            });

                                                            // Update window components
                                                            window_action.find.simple_action.set_enabled(true);

                                                            browser_action.update.activate(Some(&id));
                                                        }
                                                        Err(e) => {
                                                            // Update widget
                                                            let status_page = content.to_status_failure();
                                                            status_page.set_description(Some(&e.to_string()));

                                                            // Update meta
                                                            status.replace(Status::Failure);
                                                            title.replace(status_page.title());

                                                            // Update window
                                                            browser_action.update.activate(Some(&id));
                                                        },
                                                    }
                                                }
                                            }
                                        );
                                    },
                                    "image/png" | "image/gif" | "image/jpeg" | "image/webp" => {
                                        // Final image size unknown, show loading widget
                                        let status_page = content.to_status_loading(
                                            Some(Duration::from_secs(1)) // show if download time > 1 second
                                        );

                                        // Asynchronously move `InputStream` data from `SocketConnection` into the local `MemoryInputStream`
                                        // this action allows to count the bytes for loading widget and validate max size for incoming data
                                        gemini::gio::memory_input_stream::from_stream_async(
                                            response.connection.stream(),
                                            cancellable.clone(),
                                            Priority::DEFAULT,
                                            0x400, // 1024 bytes per chunk, optional step for images download tracking
                                            0xA00000, // 10M bytes max to prevent memory overflow if server play with promises
                                            move |_, total| {
                                                // Update loading progress
                                                status_page.set_description(
                                                    Some(&gformat!("Download: {total} bytes"))
                                                );
                                            },
                                            {
                                                let browser_action = browser_action.clone();
                                                let cancellable = cancellable.clone();
                                                let content = content.clone();
                                                let id = id.clone();
                                                let status = status.clone();
                                                let title = title.clone();
                                                let uri = uri.clone();
                                                move |result| match result {
                                                    Ok((memory_input_stream, _)) => {
                                                        Pixbuf::from_stream_async(
                                                            &memory_input_stream,
                                                            Some(&cancellable),
                                                            move |result| {
                                                                // Process buffer data
                                                                match result {
                                                                    Ok(buffer) => {
                                                                        // Update page meta
                                                                        status.replace(Status::Success);
                                                                        title.replace(uri_to_title(&uri));

                                                                        // Update page content
                                                                        content.to_image(&Texture::for_pixbuf(&buffer));

                                                                        // Update window components
                                                                        browser_action.update.activate(Some(&id));
                                                                    }
                                                                    Err(e) => {
                                                                        // Update widget
                                                                        let status_page = content.to_status_failure();
                                                                        status_page.set_description(Some(e.message()));

                                                                        // Update meta
                                                                        status.replace(Status::Failure);
                                                                        title.replace(status_page.title());
                                                                    }
                                                                }
                                                            }
                                                        );
                                                    },
                                                    Err(e) => {
                                                        // Update widget
                                                        let status_page = content.to_status_failure();
                                                        status_page.set_description(Some(&e.to_string()));

                                                        // Update meta
                                                        status.replace(Status::Failure);
                                                        title.replace(status_page.title());
                                                    }
                                                }
                                                }
                                            );
                                    },
                                    mime => {
                                        // Init children widget
                                        let status_page = content.to_status_mime(
                                            mime,
                                            Some((tab_action.clone(), navigation.request.download()))
                                        );

                                        // Update page meta
                                        status.replace(Status::Failure);
                                        title.replace(status_page.title());

                                        // Update window
                                        browser_action.update.activate(Some(&id));
                                    },
                                }
                            }
                        },
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-30-temporary-redirection
                        response::meta::Status::Redirect |
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-31-permanent-redirection
                        response::meta::Status::PermanentRedirect => {
                            // Extract redirection URL from response data
                            match response.meta.data {
                                Some(unresolved_url) => {
                                    // New URL from server MAY to be relative (according to the protocol specification),
                                    // resolve to absolute URI gobject using current request as the base for parser:
                                    // https://docs.gtk.org/glib/type_func.Uri.resolve_relative.html
                                    match Uri::resolve_relative(
                                        Some(&uri.to_string()),
                                        unresolved_url.value.as_str(),
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
                                                        status.replace(Status::Failure);
                                                        title.replace(gformat!("Oops"));

                                                        // Show placeholder with manual confirmation to continue @TODO status page?
                                                        content.to_text_gemini(
                                                            &uri,
                                                            &gformat!(
                                                                "# Redirect issue\n\nExternal redirects not allowed by protocol\n\nContinue:\n\n=> {}",
                                                                resolved_uri.to_string()
                                                            )
                                                        );
                                                    // Client MUST limit the number of redirects they follow to 5 (by protocol specification)
                                                    } else if redirect.count() > 5 {
                                                        // Update meta
                                                        status.replace(Status::Failure);
                                                        title.replace(gformat!("Oops"));

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
                                                        redirect.add(
                                                            // skip query and fragment by protocol requirements
                                                            // @TODO review fragment specification
                                                            Uri::parse(&resolved_uri.to_string_partial(
                                                                UriHideFlags::FRAGMENT | UriHideFlags::QUERY
                                                            ), UriFlags::NONE).unwrap(), // @TODO handle
                                                            // referrer
                                                            Some(uri.clone()),
                                                            // set follow policy based on status code
                                                            matches!(response.meta.status, response::meta::Status::PermanentRedirect),
                                                        );

                                                        status.replace(Status::Redirect); // @TODO is this status really wanted here?
                                                        title.replace(gformat!("Redirect"));

                                                        // Reload page to apply redirection (without history record request)
                                                        tab_action.load.activate(None, false);
                                                    }
                                                },
                                                Err(e) => {
                                                    // Update widget
                                                    let status_page = content.to_status_failure();
                                                    status_page.set_description(Some(&e.to_string()));

                                                    // Update meta
                                                    status.replace(Status::Failure);
                                                    title.replace(status_page.title());
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            // Update widget
                                            let status_page = content.to_status_failure();
                                            status_page.set_description(Some(&e.to_string()));

                                            // Update meta
                                            status.replace(Status::Failure);
                                            title.replace(status_page.title());
                                        },
                                    }
                                },
                                None => {
                                    // Update widget
                                    let status_page = content.to_status_failure();
                                    status_page.set_description(Some("Redirection target not defined"));

                                    // Update meta
                                    status.replace(Status::Failure);
                                    title.replace(status_page.title());
                                },
                            }

                            browser_action.update.activate(Some(&id));
                        },
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-60
                        response::meta::Status::CertificateRequest |
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-61-certificate-not-authorized
                        response::meta::Status::CertificateUnauthorized |
                        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-62-certificate-not-valid
                        response::meta::Status::CertificateInvalid => {
                            // Add history record
                            if is_history {
                                snap_history(&profile, &navigation, Some(&uri));
                            }

                            // Update widget
                            let status_page = content.to_status_identity();

                            status_page.set_description(Some(&match response.meta.data {
                                Some(data) => data.value,
                                None => match response.meta.status {
                                    response::meta::Status::CertificateUnauthorized => gformat!("Certificate not authorized"),
                                    response::meta::Status::CertificateInvalid => gformat!("Certificate not valid"),
                                    _ => gformat!("Client certificate required")
                                },
                            }));

                            // Update meta
                            status.replace(Status::Success);
                            title.replace(status_page.title());

                            // Update window
                            browser_action.update.activate(Some(&id));
                        }
                        _ => {
                            // Add history record
                            if is_history {
                                snap_history(&profile, &navigation, Some(&uri));
                            }

                            // Update widget
                            let status_page = content.to_status_failure();

                            status_page.set_description(Some(&match response.meta.data {
                                Some(data) => data.value,
                                None => gformat!("Status code not supported"),
                            }));

                            // Update meta
                            status.replace(Status::Failure);
                            title.replace(status_page.title());

                            // Update window
                            browser_action.update.activate(Some(&id));
                        }
                    }
                },
                Err(e) => {
                    // Add history record
                    if is_history {
                        snap_history(&profile, &navigation, Some(&uri));
                    }

                    // Update widget
                    let status_page = content.to_status_failure();
                    status_page.set_description(Some(&e.to_string()));

                    // Update meta
                    status.replace(Status::Failure);
                    title.replace(status_page.title());

                    // Update window
                    browser_action.update.activate(Some(&id));
                }
            }
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
    let title = uri.path();
    if title.split('/').last().unwrap_or_default().is_empty() {
        match uri.host() {
            Some(host) => host,
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

/// Make new history record in related components
/// * optional [Uri](https://docs.gtk.org/glib/struct.Uri.html) reference wanted only for performance reasons, to not parse it twice
fn snap_history(profile: &Profile, navigation: &Navigation, uri: Option<&Uri>) {
    let request = navigation.request.widget.entry.text();

    // Add new record into the global memory index (used in global menu)
    // * if the `Uri` is `None`, try parse it from `request`
    match uri {
        Some(uri) => profile.history.memory.request.set(uri.clone()),
        None => {
            // this case especially useful for some routes that contain redirects
            // maybe some parental optimization wanted @TODO
            if let Some(uri) = navigation.request.uri() {
                profile.history.memory.request.set(uri);
            }
        }
    }

    // Add new record into the page navigation history
    if match navigation.history.current() {
        Some(current) => current != request, // apply additional filters
        None => true,
    } {
        navigation.history.add(request, true)
    }
}
