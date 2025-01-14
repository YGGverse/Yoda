mod database;

use adw::ApplicationWindow;
use gtk::{
    gio::SimpleActionGroup,
    glib::GString,
    prelude::{GtkWindowExt, IsA, WidgetExt},
};
use sqlite::Transaction;

// Default options
const DEFAULT_HEIGHT: i32 = 480;
const DEFAULT_WIDTH: i32 = 640;
const MAXIMIZED: bool = false;

pub struct Widget {
    pub application_window: ApplicationWindow,
}

impl Widget {
    // Construct
    pub fn new(
        content: &impl IsA<gtk::Widget>,
        action_groups: &[(&GString, SimpleActionGroup)],
    ) -> Self {
        // Init GTK
        let application_window = ApplicationWindow::builder()
            .content(content)
            .default_height(DEFAULT_HEIGHT)
            .default_width(DEFAULT_WIDTH)
            .maximized(MAXIMIZED)
            .build();

        // Register actions
        for (name, group) in action_groups {
            application_window.insert_action_group(name, Some(group));
        }

        // Return new struct
        Self { application_window }
    }

    // Actions
    pub fn clean(&self, transaction: &Transaction, app_browser_id: i64) -> Result<(), String> {
        match database::select(transaction, app_browser_id) {
            Ok(records) => {
                for record in records {
                    match database::delete(transaction, record.id) {
                        Ok(_) => {
                            // Delegate clean action to childs
                            // nothing yet..
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn restore(&self, transaction: &Transaction, app_browser_id: i64) -> Result<(), String> {
        match database::select(transaction, app_browser_id) {
            Ok(records) => {
                for record in records {
                    // Restore widget
                    self.application_window.set_maximized(record.is_maximized);
                    self.application_window
                        .set_default_size(record.default_width, record.default_height);

                    // Delegate restore action to childs
                    // nothing yet..
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn save(&self, transaction: &Transaction, app_browser_id: i64) -> Result<(), String> {
        match database::insert(
            transaction,
            app_browser_id,
            self.application_window.default_width(),
            self.application_window.default_height(),
            self.application_window.is_maximized(),
        ) {
            Ok(_) => {
                // Delegate save action to childs
                // let id = self.database.last_insert_id(transaction);
                // nothing yet..
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }
}

// Tools
pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    // nothing yet..

    // Success
    Ok(())
}
