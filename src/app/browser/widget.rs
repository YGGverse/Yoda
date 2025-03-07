mod database;

use super::Window;
use adw::ApplicationWindow;
use anyhow::Result;
use gtk::{
    gio::SimpleActionGroup,
    glib::GString,
    prelude::{GtkWindowExt, WidgetExt},
};
use sqlite::Transaction;
use std::rc::Rc;

// Default options
const DEFAULT_HEIGHT: i32 = 480;
const DEFAULT_WIDTH: i32 = 640;
const MAXIMIZED: bool = false;

pub struct Widget {
    pub application_window: ApplicationWindow,
}

impl Widget {
    // Construct
    pub fn new(window: &Rc<Window>, action_groups: &[(&GString, SimpleActionGroup)]) -> Self {
        // Init GTK
        let application_window = ApplicationWindow::builder()
            .content(&window.g_box)
            .default_height(DEFAULT_HEIGHT)
            .default_width(DEFAULT_WIDTH)
            .maximized(MAXIMIZED)
            .build();

        // Register actions
        for (name, group) in action_groups {
            application_window.insert_action_group(name, Some(group));
        }

        // Connect back/forward navigation buttons @TODO use constant
        application_window.add_controller({
            let controller = gtk::GestureClick::builder().button(8).build();
            controller.connect_pressed({
                let window = window.clone();
                move |_, _, _, _| window.tab.history_back(None)
            });
            controller
        });

        application_window.add_controller({
            let controller = gtk::GestureClick::builder().button(9).build();
            controller.connect_pressed({
                let window = window.clone();
                move |_, _, _, _| window.tab.history_forward(None)
            });
            controller
        });

        // Return new struct
        Self { application_window }
    }

    // Actions
    pub fn clean(&self, transaction: &Transaction, app_browser_id: i64) -> Result<()> {
        for record in database::select(transaction, app_browser_id)? {
            database::delete(transaction, record.id)?;
        }
        Ok(())
    }

    pub fn restore(&self, transaction: &Transaction, app_browser_id: i64) -> Result<()> {
        for record in database::select(transaction, app_browser_id)? {
            // Restore widget
            self.application_window.set_maximized(record.is_maximized);
            self.application_window
                .set_default_size(record.default_width, record.default_height);

            // Delegate restore action to childs
            // nothing yet..
        }
        Ok(())
    }

    pub fn save(&self, transaction: &Transaction, app_browser_id: i64) -> Result<()> {
        database::insert(
            transaction,
            app_browser_id,
            self.application_window.default_width(),
            self.application_window.default_height(),
            self.application_window.is_maximized(),
        )?;
        Ok(())
    }
}

// Tools
pub fn migrate(tx: &Transaction) -> Result<()> {
    // Migrate self components
    database::init(tx)?;
    // Delegate migration to childs
    // nothing yet..
    // Success
    Ok(())
}
