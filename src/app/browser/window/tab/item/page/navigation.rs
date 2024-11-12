mod bookmark;
mod database;
mod history;
mod home;
mod reload;
mod request;
mod widget;

use bookmark::Bookmark;
use history::History;
use home::Home;
use reload::Reload;
use request::Request;
use widget::Widget;

use crate::app::browser::window::tab::item::Action as TabAction;
use crate::app::browser::window::Action as WindowAction;
use crate::app::browser::Action as BrowserAction;
use gtk::prelude::EditableExt;
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
    pub fn new(
        browser_action: Rc<BrowserAction>,
        window_action: Rc<WindowAction>,
        tab_action: Rc<TabAction>,
    ) -> Self {
        // Init components
        let home = Rc::new(Home::new(window_action.clone()));
        let history = Rc::new(History::new(window_action.clone()));
        let reload = Rc::new(Reload::new(window_action));
        let request = Rc::new(Request::new(browser_action, tab_action));
        let bookmark = Rc::new(Bookmark::new());

        // Init widget
        let widget = Rc::new(Widget::new(
            home.widget().gobject(),
            history.widget().gobject(),
            reload.widget().gobject(),
            request.widget().gobject(),
            bookmark.widget().gobject(),
        ));

        // Done
        Self {
            widget,
            home,
            history,
            reload,
            request,
            bookmark,
        }
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
        match database::records(transaction, app_browser_window_tab_item_page_id) {
            Ok(records) => {
                for record in records {
                    match database::delete(transaction, &record.id) {
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
        match database::records(transaction, app_browser_window_tab_item_page_id) {
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
        match database::add(transaction, app_browser_window_tab_item_page_id) {
            Ok(_) => {
                let id = database::last_insert_id(transaction);

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
    if let Err(e) = database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    request::migrate(tx)?;

    // Success
    Ok(())
}
