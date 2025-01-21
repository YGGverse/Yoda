mod content;
mod database;
mod error;
mod input;
mod navigation;
mod search;
pub mod status;
mod widget;

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
    glib::GString,
    prelude::{EditableExt, EntryExt},
};
use sqlite::Transaction;
use std::{cell::RefCell, rc::Rc};

pub struct Page {
    pub id: Rc<GString>,
    pub profile: Rc<Profile>,
    pub status: Rc<RefCell<Status>>,
    pub title: Rc<RefCell<GString>>,
    // Actions
    pub browser_action: Rc<BrowserAction>,
    pub tab_action: Rc<TabAction>,
    pub window_action: Rc<WindowAction>,
    // Components
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
        // Update children components
        self.navigation.update();
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
                    if let Some(uri) = self.navigation.request.as_uri() {
                        self.profile.history.memory.request.set(uri);
                    }
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
        let progress_fraction = self.navigation.request.widget.entry.progress_fraction();
        progress_fraction > 0.0 && progress_fraction < 1.0
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
