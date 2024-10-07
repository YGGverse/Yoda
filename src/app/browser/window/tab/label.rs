mod database;
mod pin;
mod title;
mod widget;

use database::Database;
use pin::Pin;
use sqlite::Transaction;
use title::Title;
use widget::Widget;

use gtk::{glib::GString, Box};
use std::sync::Arc;

pub struct Label {
    // Components
    pin: Arc<Pin>,
    title: Arc<Title>,
    // GTK
    widget: Arc<Widget>,
}

impl Label {
    // Construct
    pub fn new(name: GString, is_pinned: bool) -> Label {
        // Components
        let pin = Arc::new(Pin::new(is_pinned));
        let title = Arc::new(Title::new());

        // GTK
        let widget = Arc::new(Widget::new(name, pin.gobject(), title.gobject()));

        // Result
        Self { pin, title, widget }
    }

    // Actions
    pub fn clean(&self, tx: &Transaction, app_browser_window_tab_id: &i64) {
        match Database::records(tx, app_browser_window_tab_id) {
            Ok(records) => {
                for record in records {
                    match Database::delete(tx, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to childs
                            // nothing yet..
                        }
                        Err(e) => todo!("{e}"),
                    }
                }
            }
            Err(e) => todo!("{e}"),
        }
    }

    pub fn restore(&self, tx: &Transaction, app_browser_window_tab_id: &i64) {
        match Database::records(tx, app_browser_window_tab_id) {
            Ok(records) => {
                for record in records {
                    self.pin(record.is_pinned);

                    // Delegate restore action to childs
                    // nothing yet..
                }
            }
            Err(e) => todo!("{e}"),
        }
    }

    pub fn save(&self, tx: &Transaction, app_browser_window_tab_id: &i64) {
        match Database::add(tx, app_browser_window_tab_id, &self.is_pinned()) {
            Ok(_) => {
                // Delegate save action to childs
                // nothing yet..
            }
            Err(e) => todo!("{e}"),
        }
    }

    pub fn update(&self, title: Option<&GString>) {
        self.title.update(title);
        self.widget.update(title);
    }

    // Setters
    pub fn pin(&self, is_pinned: bool) {
        self.pin.pin(is_pinned);
        self.title.pin(is_pinned);
    }

    // Getters
    pub fn is_pinned(&self) -> bool {
        self.pin.is_pinned() // @TODO
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
        // nothing yet..

        // Success
        Ok(())
    }
}
