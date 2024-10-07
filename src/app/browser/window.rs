mod database;
mod tab;
mod widget;

use database::Database;
use sqlite::Transaction;
use tab::Tab;
use widget::Widget;

use std::sync::Arc;

use gtk::{gio::SimpleAction, glib::GString, Box};

pub struct Window {
    tab: Arc<Tab>,
    widget: Arc<Widget>,
}

impl Window {
    // Construct
    pub fn new(
        // Actions
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_update: Arc<SimpleAction>,
    ) -> Self {
        // Init components
        let tab = Arc::new(Tab::new(
            action_tab_page_navigation_base,
            action_tab_page_navigation_history_back,
            action_tab_page_navigation_history_forward,
            action_tab_page_navigation_reload,
            action_update,
        ));
        tab.activate(tab.clone());

        // GTK
        let widget = Arc::new(Widget::new(tab.gobject()));

        // Init struct
        Self { tab, widget }
    }

    // Actions
    pub fn tab_append(&self, tab_page_navigation_request_text: Option<GString>) {
        self.tab.append(tab_page_navigation_request_text, true);
    }

    pub fn tab_page_navigation_base(&self) {
        self.tab.page_navigation_base();
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

    pub fn update(&self) {
        self.tab.update();
    }

    pub fn clean(&self, tx: &Transaction, app_browser_id: &i64) {
        match Database::records(tx, app_browser_id) {
            Ok(records) => {
                for record in records {
                    match Database::delete(tx, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to childs
                            self.tab.clean(tx, &record.id);
                        }
                        Err(e) => todo!("{e}"),
                    }
                }
            }
            Err(e) => todo!("{e}"),
        }
    }

    pub fn restore(&self, tx: &Transaction, app_browser_id: &i64) {
        match Database::records(tx, app_browser_id) {
            Ok(records) => {
                for record in records {
                    // Delegate restore action to childs
                    self.tab.restore(tx, &record.id);
                }
            }
            Err(e) => todo!("{e}"),
        }
    }

    pub fn save(&self, tx: &Transaction, app_browser_id: &i64) {
        match Database::add(tx, app_browser_id) {
            Ok(_) => {
                // Delegate save action to childs
                self.tab.save(tx, &Database::last_insert_id(tx));
            }
            Err(e) => todo!("{e}"),
        }
    }

    // Getters
    pub fn tab_page_title(&self) -> Option<GString> {
        self.tab.page_title()
    }

    pub fn tab_page_description(&self) -> Option<GString> {
        self.tab.page_description()
    }

    pub fn gobject(&self) -> &Box {
        &self.widget.gobject()
    }

    // Tools
    pub fn migrate(tx: &Transaction) -> Result<(), String> {
        // Migrate self components
        if let Err(e) = Database::init(&tx) {
            return Err(e.to_string());
        }

        // Delegate migration to childs
        if let Err(e) = Tab::migrate(&tx) {
            return Err(e.to_string());
        }

        // Success
        Ok(())
    }
}
