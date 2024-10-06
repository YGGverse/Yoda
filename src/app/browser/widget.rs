mod database;

use database::Database;

use gtk::{prelude::GtkWindowExt, ApplicationWindow, Box, HeaderBar};
use sqlite::{Connection, Transaction};
use std::sync::{Arc, RwLock};

// Default options
const DEFAULT_HEIGHT: i32 = 480;
const DEFAULT_WIDTH: i32 = 640;
const MAXIMIZED: bool = false;

pub struct Widget {
    database: Arc<Database>,
    gobject: ApplicationWindow,
}

impl Widget {
    // Construct
    pub fn new(
        profile_database_connection: Arc<RwLock<Connection>>,
        titlebar: &HeaderBar,
        child: &Box,
    ) -> Self {
        // Init database
        let database = {
            // Init writable database connection
            let mut connection = match profile_database_connection.write() {
                Ok(connection) => connection,
                Err(e) => todo!("{e}"),
            };

            // Init new transaction
            let transaction = match connection.transaction() {
                Ok(transaction) => transaction,
                Err(e) => todo!("{e}"),
            };

            // Init database structure
            match Database::init(&transaction) {
                Ok(database) => match transaction.commit() {
                    Ok(_) => Arc::new(database),
                    Err(e) => todo!("{e}"),
                },
                Err(e) => todo!("{e}"),
            }
        };

        // Init GTK
        let gobject = ApplicationWindow::builder()
            .titlebar(titlebar)
            .child(child)
            .default_height(DEFAULT_HEIGHT)
            .default_width(DEFAULT_WIDTH)
            .maximized(MAXIMIZED)
            .build();

        // Return new struct
        Self { database, gobject }
    }

    // Actions
    pub fn clean(&self, tx: &Transaction, app_browser_id: &i64) {
        match self.database.records(tx, app_browser_id) {
            Ok(records) => {
                for record in records {
                    match self.database.delete(tx, &record.id) {
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
        match self.database.records(tx, app_browser_id) {
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
        match self.database.add(
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
}
