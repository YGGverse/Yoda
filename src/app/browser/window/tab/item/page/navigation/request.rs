mod database;
mod test;
mod widget;

use widget::Widget;

use crate::app::browser::{window::tab::item::Action as TabAction, Action as BrowserAction};
use gtk::{
    glib::{gformat, GString, Uri, UriFlags},
    prelude::EditableExt,
};
use sqlite::Transaction;
use std::rc::Rc;

// Main
pub struct Request {
    pub widget: Rc<Widget>,
}

impl Request {
    // Constructors

    /// Build new `Self`
    pub fn build((browser_action, tab_action): (&Rc<BrowserAction>, &Rc<TabAction>)) -> Self {
        Self {
            widget: Rc::new(Widget::build((browser_action, tab_action))),
        }
    }

    // Actions

    pub fn update(&self, is_identity_active: bool) {
        self.widget.update(is_identity_active);
    }

    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_navigation_id: &i64,
    ) -> Result<(), String> {
        match database::select(transaction, app_browser_window_tab_item_page_navigation_id) {
            Ok(records) => {
                for record in records {
                    match database::delete(transaction, &record.id) {
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
        match database::select(transaction, app_browser_window_tab_item_page_navigation_id) {
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
        match database::insert(transaction, app_browser_window_tab_item_page_navigation_id) {
            Ok(_) => {
                let id = database::last_insert_id(transaction);

                // Delegate save action to childs
                self.widget.save(transaction, &id)?;
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    // Setters

    pub fn into_download(&self) {
        self.widget.entry.set_text(&self.download());
    }

    pub fn into_source(&self) {
        self.widget.entry.set_text(&self.source());
    }

    // Getters

    /// Try get current request value as [Uri](https://docs.gtk.org/glib/struct.Uri.html)
    /// * `strip_prefix` on parse
    pub fn as_uri(&self) -> Option<Uri> {
        match Uri::parse(&strip_prefix(self.widget.entry.text()), UriFlags::NONE) {
            Ok(uri) => Some(uri),
            _ => None,
        }
    }

    /// Get current request value without system prefix
    /// * the `prefix` is not `scheme`
    pub fn strip_prefix(&self) -> GString {
        strip_prefix(self.widget.entry.text())
    }

    /// Get request value in `download:` format
    pub fn download(&self) -> GString {
        gformat!("download:{}", self.strip_prefix())
    }

    /// Get request value in `source:` format
    pub fn source(&self) -> GString {
        gformat!("source:{}", self.strip_prefix())
    }
}

// Tools

/// Strip system prefix from request string
/// * the `prefix` is not `scheme`
pub fn strip_prefix(mut request: GString) -> GString {
    if let Some(postfix) = request.strip_prefix("source:") {
        request = postfix.into()
    };

    if let Some(postfix) = request.strip_prefix("download:") {
        request = postfix.into()
    };

    request
} // @TODO move prefix features to page client

pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    widget::migrate(tx)?;

    // Success
    Ok(())
}
