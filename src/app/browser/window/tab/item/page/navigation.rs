mod bookmark;
mod database;
mod history;
mod home;
mod reload;
mod request;
mod widget;

use bookmark::Bookmark;
use database::Database;
use history::History;
use home::Home;
use reload::Reload;
use request::Request;
use widget::Widget;

use gtk::{gio::SimpleAction, glib::GString, prelude::WidgetExt, Box};
use sqlite::Transaction;

use std::sync::Arc;

pub struct Navigation {
    home: Arc<Home>,
    bookmark: Arc<Bookmark>,
    history: Arc<History>,
    reload: Arc<Reload>,
    request: Arc<Request>,
    widget: Arc<Widget>,
}

impl Navigation {
    pub fn new_arc(
        action_page_home: SimpleAction,
        action_page_history_back: SimpleAction,
        action_page_history_forward: SimpleAction,
        action_page_reload: SimpleAction,
        action_update: SimpleAction,
    ) -> Arc<Self> {
        // Init components
        let home = Home::new_arc(action_page_home);
        let history = History::new_arc(action_page_history_back, action_page_history_forward);
        let reload = Reload::new_arc(action_page_reload.clone());
        let request = Request::new_arc(action_update.clone(), action_page_reload.clone());
        let bookmark = Bookmark::new_arc();

        // Init widget
        let widget = Widget::new_arc(
            home.gobject(),
            history.gobject(),
            reload.gobject(),
            request.gobject(),
            bookmark.gobject(),
        );

        // Result
        Arc::new(Self {
            widget,
            home,
            history,
            reload,
            request,
            bookmark,
        })
    }

    // Actions
    pub fn request_grab_focus(&self) {
        self.request.gobject().grab_focus();
    }

    pub fn history_add(&self, request: GString) {
        self.history.add(request, true);
    }

    pub fn history_back(&self, follow_to_index: bool) -> Option<GString> {
        self.history.back(follow_to_index)
    }

    pub fn history_current(&self) -> Option<GString> {
        self.history.current()
    }

    pub fn history_forward(&self, follow_to_index: bool) -> Option<GString> {
        self.history.forward(follow_to_index)
    }

    pub fn update(&self, progress_fraction: Option<f64>) {
        self.home.update(self.request.uri());
        self.history.update();
        self.reload.update(!self.request.is_empty());
        self.request.update(progress_fraction);
        self.bookmark.update();
    }

    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_id: &i64,
    ) -> Result<(), String> {
        match Database::records(transaction, app_browser_window_tab_item_page_id) {
            Ok(records) => {
                for record in records {
                    match Database::delete(transaction, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to the item childs
                            self.request.clean(transaction, &record.id)?;
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
        app_browser_window_tab_item_page_id: &i64,
    ) -> Result<(), String> {
        match Database::records(transaction, app_browser_window_tab_item_page_id) {
            Ok(records) => {
                for record in records {
                    // Delegate restore action to the item childs
                    self.request.restore(transaction, &record.id)?;
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn save(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_id: &i64,
    ) -> Result<(), String> {
        match Database::add(transaction, app_browser_window_tab_item_page_id) {
            Ok(_) => {
                let id = Database::last_insert_id(transaction);

                // Delegate save action to childs
                self.request.save(transaction, &id)?;
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    // Setters
    pub fn set_request_text(&self, value: &str) {
        self.request.set_text(value);
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.widget.gobject()
    }

    pub fn home_url(&self) -> Option<GString> {
        self.home.url()
    }

    pub fn request_text(&self) -> GString {
        self.request.text()
    }

    // Tools
    pub fn migrate(tx: &Transaction) -> Result<(), String> {
        // Migrate self components
        if let Err(e) = Database::init(&tx) {
            return Err(e.to_string());
        }

        // Delegate migration to childs
        Request::migrate(tx)?;

        // Success
        Ok(())
    }
}
