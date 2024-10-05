mod database;

use database::Database;

use gtk::{prelude::GtkWindowExt, ApplicationWindow, Box, HeaderBar};
use std::sync::Arc;

// Default options
const DEFAULT_HEIGHT: i32 = 480;
const DEFAULT_WIDTH: i32 = 640;
const MAXIMIZED: bool = false;

pub struct Widget {
    database: Arc<Database>,
    application_window: ApplicationWindow,
}

impl Widget {
    // Construct
    pub fn new(
        profile_database_connection: Arc<sqlite::Connection>,
        titlebar: &HeaderBar,
        child: &Box,
    ) -> Self {
        // Init database
        let database = match Database::init(profile_database_connection) {
            Ok(database) => Arc::new(database),
            Err(error) => panic!("{error}"), // @TODO
        };

        // Init GTK
        let application_window = ApplicationWindow::builder()
            .titlebar(titlebar)
            .child(child)
            .default_height(DEFAULT_HEIGHT)
            .default_width(DEFAULT_WIDTH)
            .maximized(MAXIMIZED)
            .build();

        // Return new struct
        Self {
            database,
            application_window,
        }
    }

    // Actions
    pub fn clean(&self, app_browser_id: i64) {
        match self.database.records(app_browser_id) {
            Ok(records) => {
                for record in records {
                    match self.database.delete(record.id) {
                        Ok(_) => {
                            // Delegate clean action to childs
                            // nothing yet..
                        }
                        Err(error) => panic!("{error}"), // @TODO
                    }
                }
            }
            Err(error) => panic!("{error}"), // @TODO
        }
    }

    pub fn restore(&self) {
        // @TODO
    }

    pub fn save(&self, app_browser_id: i64) {
        match self.database.add(
            app_browser_id,
            self.application_window.default_width(),
            self.application_window.default_height(),
            self.application_window.is_maximized(),
        ) {
            Ok(_) => {
                // Delegate save action to childs
                // let id = self.database.last_insert_id();
                // nothing yet..
            }
            Err(error) => panic!("{error}"), // @TODO
        }
    }

    // Getters
    pub fn application_window(&self) -> &ApplicationWindow {
        &self.application_window
    }
}
