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

use gtk::{gio::SimpleAction, glib::GString, Box};

pub struct Window {
    header: Arc<Header>,
    tab: Arc<Tab>,
    widget: Arc<Widget>,
}

impl Window {
    // Construct
    pub fn new(
        // Actions
        action_tool_debug: Arc<SimpleAction>,
        action_tool_profile_directory: Arc<SimpleAction>,
        action_quit: Arc<SimpleAction>,
        action_update: Arc<SimpleAction>,
        action_tab_append: Arc<SimpleAction>,
        action_tab_close: Arc<SimpleAction>,
        action_tab_close_all: Arc<SimpleAction>,
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_tab_pin: Arc<SimpleAction>,
    ) -> Self {
        // Init components
        let header = Arc::new(Header::new(
            action_tool_debug.clone(),
            action_tool_profile_directory.clone(),
            action_quit.clone(),
            action_tab_append.clone(),
            action_tab_close.clone(),
            action_tab_close_all.clone(),
            action_tab_page_navigation_base.clone(),
            action_tab_page_navigation_history_back.clone(),
            action_tab_page_navigation_history_forward.clone(),
            action_tab_page_navigation_reload.clone(),
            action_tab_pin.clone(),
        ));

        let tab = Arc::new(Tab::new(
            action_tab_page_navigation_base,
            action_tab_page_navigation_history_back,
            action_tab_page_navigation_history_forward,
            action_tab_page_navigation_reload,
            action_update,
        ));
        tab.activate(tab.clone());

        // GTK
        let widget = Arc::new(Widget::new(header.gobject(), tab.gobject()));

        // Init struct
        Self {
            header,
            tab,
            widget,
        }
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
        // Update header
        let title = match self.tab.page_title() {
            Some(value) => value,
            None => GString::new(), // @TODO
        };

        let subtitle = match self.tab.page_description() {
            Some(value) => value,
            None => GString::new(), // @TODO
        };

        self.header.update(title.as_str(), subtitle.as_str());

        self.tab.update();
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

    // Getters
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
        Tab::migrate(&tx)?;

        // Success
        Ok(())
    }
}
