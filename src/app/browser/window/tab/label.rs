mod database;
mod pin;
mod title;
mod widget;

use database::Database;
use pin::Pin;
use sqlite::{Connection, Transaction};
use title::Title;
use widget::Widget;

use gtk::{glib::GString, Box};
use std::sync::{Arc, RwLock};

pub struct Label {
    database: Arc<Database>,
    // Components
    pin: Arc<Pin>,
    title: Arc<Title>,
    // GTK
    widget: Arc<Widget>,
}

impl Label {
    // Construct
    pub fn new(
        profile_database_connection: Arc<RwLock<Connection>>,
        name: GString,
        is_pinned: bool,
    ) -> Label {
        // Init database
        let database = {
            /* @TODO init outside
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
            } */

            Arc::new(Database::new())
        };

        // Components
        let pin = Arc::new(Pin::new(is_pinned));
        let title = Arc::new(Title::new());

        // GTK
        let widget = Arc::new(Widget::new(name, pin.gobject(), title.gobject()));

        // Result
        Self {
            database,
            pin,
            title,
            widget,
        }
    }

    // Actions
    pub fn clean(&self, tx: &Transaction, app_browser_window_tab_id: &i64) {
        match self.database.records(tx, app_browser_window_tab_id) {
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

    pub fn restore(&self, tx: &Transaction, app_browser_window_tab_id: &i64) {
        match self.database.records(tx, app_browser_window_tab_id) {
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
        match self
            .database
            .add(tx, app_browser_window_tab_id, &self.is_pinned())
        {
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
}
