mod client; // @TODO complete new router implementation
mod content;
mod database;
mod error;
mod input;
mod navigation;
mod search;
mod status;
mod widget;

use client::Client;
use content::Content;
use error::Error;
use input::Input;
use navigation::Navigation;
use search::Search;
use status::Status;
use widget::Widget;

use super::{Action as TabAction, BrowserAction, Profile, WindowAction};
use crate::tool::now;

use gtk::{
    gdk::Texture,
    gdk_pixbuf::Pixbuf,
    glib::{GString, Priority, Uri},
    prelude::{EditableExt, FileExt},
};
use sqlite::Transaction;
use std::{cell::RefCell, path::MAIN_SEPARATOR, rc::Rc, time::Duration};

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

        let status = Rc::new(RefCell::new(Status::New { time: now() }));

        let client = Rc::new(Client::init(profile, {
            let id = id.clone();
            let status = status.clone();
            let update = browser_action.update.clone();
            move |this| {
                status.replace(Status::Client(this));
                update.activate(Some(&id));
            }
        }));

        // Done
        Self {
            id: id.clone(),
            profile: profile.clone(),
            title: Rc::new(RefCell::new("New page".into())),
            // Actions
            browser_action: browser_action.clone(),
            tab_action: tab_action.clone(),
            window_action: window_action.clone(),
            // Components
            client,
            status,
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

    /// Request `Escape` action for all page components
    pub fn escape(&self) {
        self.search.hide()
    }

    /// Toggle `Find` widget
    pub fn find(&self) {
        self.search.show()
    }

    /// Update `Self` witch children components
    pub fn update(&self) {
        // Update components
        self.navigation
            .update(self.status.borrow().to_progress_fraction());
        // @TODO self.content.update();
    }

    /// Cleanup session for `Self`
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
        self.status.replace(Status::SessionRestore { time: now() });

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
        self.status.replace(Status::SessionRestored { time: now() });

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

    /// Get `Self` loading status
    pub fn is_loading(&self) -> bool {
        match self.status.borrow().to_progress_fraction() {
            Some(progress_fraction) => progress_fraction < 1.0,
            None => false,
        }
    }
}

/// Navigate home URL (parsed from current navigation entry)
/// * this method create new history record in memory as defined in `action_page_open` action
pub fn home(page: &Rc<Page>) {
    if let Some(text) = page.navigation.home.url() {
        page.navigation.request.widget.entry.set_text(&text);
        load(page, None, false);
    }
}

/// Navigate back in history
/// * this method does not create new history record in memory
pub fn history_back(page: &Rc<Page>) {
    if let Some(text) = page.navigation.history.back(true) {
        page.navigation.request.widget.entry.set_text(&text);
        load(page, None, false);
    }
}

/// Navigate forward in history
/// * this method does not create new history record in memory
pub fn history_forward(page: &Rc<Page>) {
    if let Some(text) = page.navigation.history.forward(true) {
        page.navigation.request.widget.entry.set_text(&text);
        load(page, None, false);
    }
}

/// Page load function with recursive redirection support
pub fn load(page: &Rc<Page>, request: Option<&str>, is_history: bool) {
    // Move focus out from navigation entry
    page.browser_action
        .escape
        .activate_stateful_once(Some(page.id.as_str().into()));

    // Initially disable find action
    page.window_action.find.simple_action.set_enabled(false);

    // Reset widgets
    page.search.unset();
    page.input.unset();
    page.status.replace(Status::Loading { time: now() });
    page.title.replace("Loading..".into());
    page.browser_action.update.activate(Some(&page.id));

    if is_history {
        snap_history(&page.profile, &page.navigation, None); // @TODO
    }

    let query = match request {
        Some(query) => query,
        None => &page.navigation.request.widget.entry.text(),
    };

    page.client.request(query, {
        let page = page.clone();
        move |response| handle(&page, response)
    });
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
/// * useful as common placeholder when page title could not be detected
/// * this feature may be improved and moved outside @TODO
fn uri_to_title(uri: &Uri) -> GString {
    let path = uri.path();
    if path.split('/').last().unwrap_or_default().is_empty() {
        match uri.host() {
            Some(host) => host,
            None => "Untitled".into(),
        }
    } else {
        path
    }
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

/// Response handler for `Page`
/// * may call itself on Titan response
fn handle(page: &Rc<Page>, response: client::Response) {
    use client::{
        response::{text::Text, Certificate, Failure, Input, Redirect},
        Response,
    };
    match response {
        Response::Certificate(this) => match this {
            Certificate::Invalid {
                title: certificate_title,
            }
            | Certificate::Request {
                title: certificate_title,
            }
            | Certificate::Unauthorized {
                title: certificate_title,
            } => {
                // Update widget
                let status = page.content.to_status_identity();
                status.set_description(Some(&certificate_title));

                // Update meta
                page.status.replace(Status::Success { time: now() });
                page.title.replace(status.title());

                // Update window
                page.browser_action.update.activate(Some(&page.id));
            }
        },
        Response::Failure(this) => match this {
            Failure::Status { message } | Failure::Error { message } => {
                // Update widget
                let status = page.content.to_status_failure();
                status.set_description(Some(&message));

                // Update meta
                page.status.replace(Status::Failure { time: now() });
                page.title.replace(status.title());

                // Update window
                page.browser_action.update.activate(Some(&page.id));
            }
            Failure::Mime {
                base,
                mime,
                message,
            } => {
                // Update widget
                let status = page
                    .content
                    .to_status_mime(&mime, Some((&page.tab_action, &base)));
                status.set_description(Some(&message));

                // Update meta
                page.status.replace(Status::Failure { time: now() });
                page.title.replace(status.title());

                // Update window
                page.browser_action.update.activate(Some(&page.id));
            }
        },
        Response::Input(this) => {
            match this {
                Input::Response {
                    base,
                    title: response_title,
                } => {
                    page.input.set_new_response(
                        page.tab_action.clone(),
                        base,
                        Some(&response_title),
                        Some(1024),
                    );
                    page.title.replace(response_title);
                }
                Input::Sensitive {
                    base,
                    title: response_title,
                } => {
                    page.input.set_new_sensitive(
                        page.tab_action.clone(),
                        base,
                        Some(&response_title),
                        Some(1024),
                    );
                    page.title.replace(response_title);
                }
                Input::Titan(this) => {
                    page.input.set_new_titan(this, {
                        let page = page.clone();
                        move |result| match result {
                            Ok(response) => handle(&page, response),
                            Err(e) => {
                                let status = page.content.to_status_failure();
                                //status.set_description(Some(&e.to_string()));
                                // @TODO

                                page.status.replace(Status::Failure { time: now() });
                                page.title.replace(status.title());
                                page.browser_action.update.activate(Some(&page.id));
                            }
                        }
                    });
                    page.title.replace("Titan input".into());
                }
            }
            page.status.replace(Status::Input { time: now() });
            page.browser_action.update.activate(Some(&page.id));
        }
        Response::Redirect(this) => match this {
            Redirect::Background(uri) => load(&page, Some(&uri.to_string()), false),
            Redirect::Foreground(uri) => {
                page.navigation
                    .request
                    .widget
                    .entry
                    .set_text(&uri.to_string());
                load(&page, Some(&uri.to_string()), false);
            }
        },
        Response::Text(this) => match this {
            Text::Gemini { base, data } => {
                /* @TODO refactor features
                let widget = if is_source_request {
                    page.content.to_text_source(&data)
                } else {
                    page.content.to_text_gemini(&base, &data)
                };*/

                let widget = page.content.to_text_gemini(&base, &data);

                // Connect `TextView` widget, update `search` model
                page.search.set(Some(widget.text_view));

                // Update page meta
                page.status.replace(Status::Success { time: now() });
                page.title.replace(match widget.meta.title {
                    Some(title) => title.into(), // @TODO
                    None => uri_to_title(&base),
                });

                // Update window components
                page.window_action.find.simple_action.set_enabled(true);
                page.browser_action.update.activate(Some(&page.id));
            }
            Text::Plain { data } => todo!(),
        },
        Response::Download {
            base,
            cancellable,
            stream,
        } => {
            // Init download widget
            let status = page.content.to_status_download(
                uri_to_title(&base).trim_matches(MAIN_SEPARATOR), // grab default filename from base URI,
                // format FS entities
                &cancellable,
                {
                    let cancellable = cancellable.clone();
                    let stream = stream.clone();
                    move |file, action| {
                        match file.replace(
                            None,
                            false,
                            gtk::gio::FileCreateFlags::NONE,
                            Some(&cancellable),
                        ) {
                            Ok(file_output_stream) => {
                                // Asynchronously read [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
                                // to local [MemoryInputStream](https://docs.gtk.org/gio/class.MemoryInputStream.html)
                                // show bytes count in loading widget, validate max size for incoming data
                                // * no dependency of Gemini library here, feel free to use any other `IOStream` processor
                                ggemini::gio::file_output_stream::move_all_from_stream_async(
                                    stream.clone(),
                                    file_output_stream,
                                    cancellable.clone(),
                                    Priority::DEFAULT,
                                    (
                                        0x100000, // 1M bytes per chunk
                                        None,     // unlimited
                                        0,        // initial totals
                                    ),
                                    (
                                        // on chunk
                                        {
                                            let action = action.clone();
                                            move |_, total| {
                                                action.update.activate(&format!(
                                                    "Received {}...",
                                                    crate::tool::format_bytes(total)
                                                ))
                                            }
                                        },
                                        // on complete
                                        {
                                            let action = action.clone();
                                            move |result| match result {
                                                Ok((_, total)) => {
                                                    action.complete.activate(&format!(
                                                        "Saved to {} ({total} bytes total)",
                                                        file.parse_name()
                                                    ))
                                                }
                                                Err(e) => action.cancel.activate(&e.to_string()),
                                            }
                                        },
                                    ),
                                );
                            }
                            Err(e) => action.cancel.activate(&e.to_string()),
                        }
                    }
                },
            );

            // Update meta
            page.status.replace(Status::Success { time: now() });
            page.title.replace(status.title());

            // Update window
            page.browser_action.update.activate(Some(&page.id));
        }
        Response::Stream {
            base,
            mime,
            stream,
            cancellable,
        } => match mime.as_str() {
            // @TODO use client-side const or enum?
            "image/png" | "image/gif" | "image/jpeg" | "image/webp" => {
                // Final image size unknown, show loading widget
                let status = page.content.to_status_loading(
                    Some(Duration::from_secs(1)), // show if download time > 1 second
                );

                // Asynchronously read [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
                // to local [MemoryInputStream](https://docs.gtk.org/gio/class.MemoryInputStream.html)
                // show bytes count in loading widget, validate max size for incoming data
                // * no dependency of Gemini library here, feel free to use any other `IOStream` processor
                ggemini::gio::memory_input_stream::from_stream_async(
                    stream,
                    cancellable.clone(),
                    Priority::DEFAULT,
                    0x400, // 1024 bytes per chunk, optional step for images download tracking
                    0xA00000, // 10M bytes max to prevent memory overflow if server play with promises
                    move |_, total| {
                        // Update loading progress
                        status.set_description(Some(&format!("Download: {total} bytes")));
                    },
                    {
                        let page = page.clone();
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
                                                page.status
                                                    .replace(Status::Success { time: now() });
                                                page.title.replace(uri_to_title(&base));

                                                // Update page content
                                                page.content
                                                    .to_image(&Texture::for_pixbuf(&buffer));

                                                // Update window components
                                                page.browser_action.update.activate(Some(&page.id));
                                            }
                                            Err(e) => {
                                                // Update widget
                                                let status = page.content.to_status_failure();
                                                status.set_description(Some(e.message()));

                                                // Update meta
                                                page.status
                                                    .replace(Status::Failure { time: now() });
                                                page.title.replace(status.title());
                                            }
                                        }
                                    },
                                );
                            }
                            Err(e) => {
                                // Update widget
                                let status = page.content.to_status_failure();
                                status.set_description(Some(&e.to_string()));

                                // Update meta
                                page.status.replace(Status::Failure { time: now() });
                                page.title.replace(status.title());
                            }
                        }
                    },
                );
            }
            _ => todo!(), // unexpected
        },
    }
}
