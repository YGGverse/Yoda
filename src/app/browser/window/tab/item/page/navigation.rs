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

use crate::app::browser::action::Action as BrowserAction;
use crate::app::browser::window::action::Action as WindowAction;
use gtk::{gio::SimpleAction, prelude::EditableExt};
use sqlite::Transaction;
use std::rc::Rc;

pub struct Navigation {
    home: Rc<Home>,
    bookmark: Rc<Bookmark>,
    history: Rc<History>,
    reload: Rc<Reload>,
    request: Rc<Request>,
    widget: Rc<Widget>,
}

impl Navigation {
    pub fn new_rc(
        browser_action: Rc<BrowserAction>,
        window_action: Rc<WindowAction>,
        action_page_open: SimpleAction,
    ) -> Rc<Self> {
        // Init components
        let home = Home::new_rc(window_action.clone());
        let history = History::new_rc(window_action.clone());
        let reload = Reload::new_rc(window_action);
        let request = Request::new_rc(browser_action, action_page_open.clone());
        let bookmark = Bookmark::new_rc();

        // Init widget
        let widget = Widget::new_rc(
            home.gobject(),
            history.gobject(),
            reload.gobject(),
            request.gobject(),
            bookmark.gobject(),
        );

        // Result
        Rc::new(Self {
            widget,
            home,
            history,
            reload,
            request,
            bookmark,
        })
    }

    // Actions

    pub fn update(&self, progress_fraction: Option<f64>) {
        self.home.update(self.request.uri());
        self.history.update();
        self.reload
            .update(!self.request.widget().gobject().text().is_empty());
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

    // Getters

    pub fn home(&self) -> &Rc<Home> {
        &self.home
    }

    pub fn history(&self) -> &Rc<History> {
        &self.history
    }

    /*
    pub fn reload(&self) -> &Rc<Reload> {
        &self.reload
    } */

    pub fn request(&self) -> &Rc<Request> {
        &self.request
    }

    /*
    pub fn bookmark(&self) -> &Rc<Bookmark> {
        &self.bookmark
    } */

    pub fn widget(&self) -> &Rc<Widget> {
        &self.widget
    }
}

// Tools
pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = Database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    request::migrate(tx)?;

    // Success
    Ok(())
}
