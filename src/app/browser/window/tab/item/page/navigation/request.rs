mod database;
mod widget;

use database::Database;
use widget::Widget;

use crate::app::browser::action::Action as BrowserAction;
use gtk::{
    gio::SimpleAction,
    glib::{Uri, UriFlags},
    prelude::EditableExt,
    Entry,
};
use sqlite::Transaction;
use std::rc::Rc;

// Main
pub struct Request {
    widget: Rc<Widget>,
}

impl Request {
    // Construct
    pub fn new_rc(
        // Actions
        browser_action: Rc<BrowserAction>,
        action_page_reload: SimpleAction, // @TODO local `action_page_open`?
    ) -> Rc<Self> {
        Rc::new(Self {
            widget: Widget::new_rc(browser_action, action_page_reload),
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

    // Getters
    pub fn gobject(&self) -> &Entry {
        self.widget.gobject()
    }

    pub fn widget(&self) -> &Rc<Widget> {
        &self.widget
    }

    pub fn uri(&self) -> Option<Uri> {
        match Uri::parse(&self.widget.gobject().text(), UriFlags::NONE) {
            Ok(uri) => Some(uri),
            _ => None,
        }
    }
}

// Tools
pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = Database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    widget::migrate(tx)?;

    // Success
    Ok(())
}
