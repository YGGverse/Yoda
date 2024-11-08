mod database;
mod header;
mod tab;
mod widget;

use database::Database;
use header::Header;
use sqlite::Transaction;
use tab::Tab;
use widget::Widget;

use crate::action::Browser as BrowserAction;
use gtk::{gio::SimpleAction, Box};
use std::rc::Rc;

pub struct Window {
    //header: Rc<Header>,
    tab: Rc<Tab>,
    widget: Rc<Widget>,
}

impl Window {
    // Construct
    pub fn new(
        // Actions
        browser_action: Rc<BrowserAction>,
        action_page_new: SimpleAction,
        action_page_close: SimpleAction,
        action_page_close_all: SimpleAction,
        action_page_home: SimpleAction,
        action_page_history_back: SimpleAction,
        action_page_history_forward: SimpleAction,
        action_page_reload: SimpleAction,
        action_page_pin: SimpleAction,
    ) -> Self {
        // Init components
        let tab = Tab::new_rc(
            browser_action.clone(),
            action_page_close.clone(),
            action_page_close_all.clone(),
            action_page_home.clone(),
            action_page_history_back.clone(),
            action_page_history_forward.clone(),
            action_page_pin.clone(),
            action_page_reload.clone(),
        );

        let header = Header::new_rc(
            // Actions
            browser_action,
            action_page_new,
            action_page_close,
            action_page_close_all,
            action_page_home,
            action_page_history_back,
            action_page_history_forward,
            action_page_reload,
            action_page_pin,
            // Widgets
            tab.gobject(),
        );

        // GTK
        let widget = Rc::new(Widget::new(header.gobject(), tab.gobject()));

        // Init struct
        Self {
            //header,
            tab,
            widget,
        }
    }

    // Actions
    pub fn tab_append(&self, page_position: Option<i32>) {
        self.tab.append(page_position);
    }

    pub fn tab_page_home(&self, page_position: Option<i32>) {
        self.tab.page_home(page_position);
    }

    pub fn tab_page_history_back(&self, page_position: Option<i32>) {
        self.tab.page_history_back(page_position);
    }

    pub fn tab_page_history_forward(&self, page_position: Option<i32>) {
        self.tab.page_history_forward(page_position);
    }

    /// Reload page at given position or selected page on `None` given
    pub fn tab_page_reload(&self, position: Option<i32>) {
        self.tab.page_reload(position);
    }

    /// Close page at given position or selected page on `None` given
    pub fn tab_close(&self, page_position: Option<i32>) {
        self.tab.close(page_position);
    }

    pub fn tab_close_all(&self) {
        self.tab.close_all();
    }

    pub fn tab_pin(&self, page_position: Option<i32>) {
        self.tab.pin(page_position);
    }

    pub fn update(&self, id: &str) {
        self.tab.update(id);
    }

    pub fn clean(&self, transaction: &Transaction, app_browser_id: &i64) -> Result<(), String> {
        match Database::records(transaction, app_browser_id) {
            Ok(records) => {
                for record in records {
                    match Database::delete(transaction, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to childs
                            self.tab.clean(transaction, &record.id)?;
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn restore(&self, transaction: &Transaction, app_browser_id: &i64) -> Result<(), String> {
        match Database::records(transaction, app_browser_id) {
            Ok(records) => {
                for record in records {
                    // Delegate restore action to childs
                    self.tab.restore(transaction, &record.id)?;
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn save(&self, transaction: &Transaction, app_browser_id: &i64) -> Result<(), String> {
        match Database::add(transaction, app_browser_id) {
            Ok(_) => {
                // Delegate save action to childs
                if let Err(e) = self
                    .tab
                    .save(transaction, &Database::last_insert_id(transaction))
                {
                    return Err(e.to_string());
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn init(&self) {
        self.tab.init();
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        self.widget.gobject()
    }
}

// Tools
pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = Database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    tab::migrate(tx)?;

    // Success
    Ok(())
}
