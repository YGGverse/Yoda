mod database;

use database::Database;

use gtk::{prelude::GtkWindowExt, ApplicationWindow, Box, HeaderBar};
use sqlite::Transaction;

// Default options
const DEFAULT_HEIGHT: i32 = 480;
const DEFAULT_WIDTH: i32 = 640;
const MAXIMIZED: bool = false;

pub struct Widget {
    gobject: ApplicationWindow,
}

impl Widget {
    // Construct
    pub fn new(titlebar: &HeaderBar, child: &Box) -> Self {
        // Init GTK
        let gobject = ApplicationWindow::builder()
            .titlebar(titlebar)
            .child(child)
            .default_height(DEFAULT_HEIGHT)
            .default_width(DEFAULT_WIDTH)
            .maximized(MAXIMIZED)
            .build();

        // Return new struct
        Self { gobject }
    }

    // Actions
    pub fn clean(&self, tx: &Transaction, app_browser_id: &i64) {
        match Database::records(tx, app_browser_id) {
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

    pub fn restore(&self, tx: &Transaction, app_browser_id: &i64) {
        match Database::records(tx, app_browser_id) {
            Ok(records) => {
                for record in records {
                    // Restore widget
                    self.gobject.set_maximized(record.is_maximized);
                    self.gobject
                        .set_default_size(record.default_width, record.default_height);

                    // Delegate restore action to childs
                    // nothing yet..
                }
            }
            Err(e) => todo!("{e}"),
        }
    }

    pub fn save(&self, tx: &Transaction, app_browser_id: &i64) {
        match Database::add(
            tx,
            app_browser_id,
            &self.gobject.default_width(),
            &self.gobject.default_height(),
            &self.gobject.is_maximized(),
        ) {
            Ok(_) => {
                // Delegate save action to childs
                // let id = self.database.last_insert_id(tx);
                // nothing yet..
            }
            Err(e) => todo!("{e}"),
        }
    }

    // Getters
    pub fn gobject(&self) -> &ApplicationWindow {
        &self.gobject
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
