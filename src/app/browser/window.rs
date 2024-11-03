mod database;
mod header;
mod tab;
mod widget;

use database::Database;
use header::Header;
use sqlite::Transaction;
use tab::Tab;
use widget::Widget;

use std::sync::Arc;

use gtk::{gio::SimpleAction, Box};

pub struct Window {
    //header: Arc<Header>,
    tab: Arc<Tab>,
    widget: Arc<Widget>,
}

impl Window {
    // Construct
    pub fn new(
        // Actions
        action_debug: SimpleAction,
        action_profile: SimpleAction,
        action_quit: SimpleAction,
        action_update: SimpleAction,
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
        let tab = Tab::new_arc(
            action_page_home.clone(),
            action_page_history_back.clone(),
            action_page_history_forward.clone(),
            action_page_reload.clone(),
            action_update.clone(),
        );

        let header = Header::new_arc(
            // Actions
            action_debug.clone(),
            action_profile.clone(),
            action_quit.clone(),
            action_page_new.clone(),
            action_page_close.clone(),
            action_page_close_all.clone(),
            action_page_home.clone(),
            action_page_history_back.clone(),
            action_page_history_forward.clone(),
            action_page_reload.clone(),
            action_page_pin.clone(),
            // Widgets
            tab.gobject(),
        );

        // GTK
        let widget = Arc::new(Widget::new(header.gobject(), tab.gobject()));

        // Init struct
        Self {
            //header,
            tab,
            widget,
        }
    }

    // Actions
    pub fn tab_append(&self, position: Option<i32>) {
        self.tab.append(position);
    }

    pub fn tab_page_navigation_home(&self) {
        self.tab.page_navigation_home();
    }

    pub fn tab_page_navigation_history_back(&self) {
        self.tab.page_navigation_history_back();
    }

    pub fn tab_page_navigation_history_forward(&self) {
        self.tab.page_navigation_history_forward();
    }

    pub fn tab_page_navigation_reload(&self) {
        self.tab.page_navigation_reload();
    }

    pub fn tab_close(&self) {
        self.tab.close();
    }

    pub fn tab_close_all(&self) {
        self.tab.close_all();
    }

    pub fn tab_pin(&self) {
        self.tab.pin();
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
        &self.widget.gobject()
    }
}

// Tools
pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = Database::init(&tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    tab::migrate(&tx)?;

    // Success
    Ok(())
}
