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
    bookmark: Rc<Bookmark>,
    history: Rc<History>,
    home: Rc<Home>,
    reload: Rc<Reload>,
    request: Rc<Request>,
    widget: Rc<Widget>,
}

impl Navigation {
    pub fn new(action: (Rc<BrowserAction>, Rc<WindowAction>, Rc<TabAction>)) -> Self {
        // Init components
        let home = Rc::new(Home::new(action.1.clone()));
        let history = Rc::new(History::new(action.1.clone()));
        let reload = Rc::new(Reload::new(action.1.clone()));
        let request = Rc::new(Request::new((action.0, action.2)));
        let bookmark = Rc::new(Bookmark::new(action.1));

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
            bookmark,
            history,
            home,
            reload,
            request,
            widget,
        }
    }

    // Actions

    pub fn update(&self, progress_fraction: Option<f64>) {
        self.bookmark.update(false); // @TODO DB from request
        self.history.update();
        self.home.update(self.request.uri());
        self.reload
            .update(!self.request.widget().gobject().text().is_empty());
        self.request.update(progress_fraction);
    }

    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_id: &i64,
    ) -> Result<(), String> {
        match database::select(transaction, app_browser_window_tab_item_page_id) {
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
        match database::select(transaction, app_browser_window_tab_item_page_id) {
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
        match database::insert(transaction, app_browser_window_tab_item_page_id) {
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
