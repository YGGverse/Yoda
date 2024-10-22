mod database;
mod widget;

use database::Database;
use widget::Widget;

use gtk::{
    gio::SimpleAction,
    glib::{GString, Uri, UriFlags},
    Entry,
};
use sqlite::Transaction;
use std::sync::Arc;

// Main
pub struct Request {
    widget: Arc<Widget>,
}

impl Request {
    // Construct
    pub fn new_arc(
        // Actions
        action_update: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>, // @TODO local `action_page_open`?
    ) -> Arc<Self> {
        Arc::new(Self {
            widget: Widget::new_arc(action_update, action_tab_page_navigation_reload),
        })
    }

    // Actions
    pub fn update(&self, progress_fraction: Option<f64>) {
        self.widget.update(progress_fraction);
    }

    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_navigation_id: &i64,
    ) -> Result<(), String> {
        match Database::records(transaction, app_browser_window_tab_item_page_navigation_id) {
            Ok(records) => {
                for record in records {
                    match Database::delete(transaction, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to the item childs
                            self.widget.clean(transaction, &record.id)?;
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
        app_browser_window_tab_item_page_navigation_id: &i64,
    ) -> Result<(), String> {
        match Database::records(transaction, app_browser_window_tab_item_page_navigation_id) {
            Ok(records) => {
                for record in records {
                    // Delegate restore action to the item childs
                    self.widget.restore(transaction, &record.id)?;
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn save(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_navigation_id: &i64,
    ) -> Result<(), String> {
        match Database::add(transaction, app_browser_window_tab_item_page_navigation_id) {
            Ok(_) => {
                let id = Database::last_insert_id(transaction);

                // Delegate save action to childs
                self.widget.save(transaction, &id)?;
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    // Setters
    pub fn set_text(&self, value: &str) {
        self.widget.set_text(value);
    }

    // Getters
    pub fn gobject(&self) -> &Entry {
        &self.widget.gobject()
    }

    pub fn is_empty(&self) -> bool {
        self.widget.is_empty()
    }

    pub fn text(&self) -> GString {
        self.widget.text()
    }

    pub fn uri(&self) -> Option<Uri> {
        match Uri::parse(&self.widget.text(), UriFlags::NONE) {
            Ok(uri) => Some(uri),
            _ => None,
        }
    }

    // Tools
    pub fn migrate(tx: &Transaction) -> Result<(), String> {
        // Migrate self components
        if let Err(e) = Database::init(tx) {
            return Err(e.to_string());
        }

        // Delegate migration to childs
        Widget::migrate(tx)?;

        // Success
        Ok(())
    }
}
