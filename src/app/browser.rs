mod database;
mod widget;
mod window;

use database::Database;
use widget::Widget;
use window::Window;

use adw::ApplicationWindow;
use gtk::{
    gio::{Cancellable, File, SimpleAction},
    prelude::GtkWindowExt,
    FileLauncher,
};
use sqlite::Transaction;
use std::{path::PathBuf, sync::Arc};

pub struct Browser {
    // Components
    // header: Arc<Header>,
    window: Arc<Window>,
    widget: Arc<Widget>,
}

impl Browser {
    // Construct
    pub fn new(
        // Extras
        profile_path: PathBuf,
        // Actions
        action_debug: SimpleAction,
        action_profile: SimpleAction,
        action_quit: SimpleAction,
        action_update: SimpleAction,
        action_page_new: SimpleAction,
        action_page_close: SimpleAction,
        action_page_close_all: SimpleAction,
        action_page_home: SimpleAction,
        action_page_history_back: SimpleAction,
        action_page_history_forward: SimpleAction,
        action_page_reload: SimpleAction,
        action_page_pin: SimpleAction,
    ) -> Browser {
        // Init components
        let window = Arc::new(Window::new(
            action_debug.clone(),
            action_profile.clone(),
            action_quit.clone(),
            action_update.clone(),
            action_page_new.clone(),
            action_page_close.clone(),
            action_page_close_all.clone(),
            action_page_home.clone(),
            action_page_history_back.clone(),
            action_page_history_forward.clone(),
            action_page_reload.clone(),
            action_page_pin.clone(),
        ));

        // Init widget
        let widget = Arc::new(Widget::new(
            window.gobject(),
            &[
                action_debug.clone(),
                action_profile.clone(),
                action_quit.clone(),
                action_update.clone(),
                action_page_new.clone(),
                action_page_close.clone(),
                action_page_close_all.clone(),
                action_page_home.clone(),
                action_page_history_back.clone(),
                action_page_history_forward.clone(),
                action_page_reload.clone(),
                action_page_pin.clone(),
            ],
        ));

        // Init events
        action_debug.connect_activate({
            let widget = widget.clone();
            move |_, _| {
                widget.gobject().emit_enable_debugging(true);
            }
        });

        action_profile.connect_activate({
            move |_, _| {
                FileLauncher::new(Some(&File::for_path(&profile_path))).launch(
                    None::<&gtk::Window>,
                    None::<&Cancellable>,
                    |result| {
                        if let Err(error) = result {
                            // @TODO
                            println!("Could not delegate launch action: {error}")
                        }
                    },
                );
            }
        });

        action_quit.connect_activate({
            let widget = widget.clone();
            move |_, _| {
                widget.gobject().close();
            }
        });

        action_update.connect_activate({
            let window = window.clone();
            move |_, id| {
                window.update(
                    id.expect("Page ID required for update action")
                        .get::<String>()
                        .expect("Parameter does not match `String`")
                        .as_str(),
                );
            }
        });

        action_page_new.connect_activate({
            let window = window.clone();
            move |_, _| {
                window.tab_append(None);
            }
        });

        action_page_close.connect_activate({
            let window = window.clone();
            move |_, _| {
                window.tab_close();
            }
        });

        action_page_close_all.connect_activate({
            let window = window.clone();
            move |_, _| {
                window.tab_close_all();
            }
        });

        action_page_home.connect_activate({
            let window = window.clone();
            move |_, _| {
                window.tab_page_navigation_home();
            }
        });

        action_page_history_back.connect_activate({
            let window = window.clone();
            move |_, _| {
                window.tab_page_navigation_history_back();
            }
        });

        action_page_history_forward.connect_activate({
            let window = window.clone();
            move |_, _| {
                window.tab_page_navigation_history_forward();
            }
        });

        action_page_reload.connect_activate({
            let window = window.clone();
            move |_, _| {
                window.tab_page_navigation_reload();
            }
        });

        action_page_pin.connect_activate({
            let window = window.clone();
            move |_, _| {
                window.tab_pin();
            }
        });

        // Return new activated `Self`
        Self {
            widget,
            // header,
            window,
        }
    }

    // Actions
    pub fn clean(&self, transaction: &Transaction, app_id: &i64) -> Result<(), String> {
        match Database::records(transaction, app_id) {
            Ok(records) => {
                for record in records {
                    match Database::delete(transaction, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to childs
                            self.window.clean(transaction, &record.id)?;
                            self.widget.clean(transaction, &record.id)?;

                            /* @TODO
                            self.header.clean(transaction, &record.id)?; */
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn restore(&self, transaction: &Transaction, app_id: &i64) -> Result<(), String> {
        match Database::records(transaction, app_id) {
            Ok(records) => {
                for record in records {
                    // Delegate restore action to childs
                    self.widget.restore(transaction, &record.id)?;
                    self.window.restore(transaction, &record.id)?;

                    /* @TODO
                    self.header.restore(transaction, &record.id)?; */
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn save(&self, transaction: &Transaction, app_id: &i64) -> Result<(), String> {
        match Database::add(transaction, app_id) {
            Ok(_) => {
                let id = Database::last_insert_id(transaction);

                // Delegate save action to childs
                self.widget.save(transaction, &id)?;
                self.window.save(transaction, &id)?;

                /* @TODO
                self.header.save(transaction, &id)?; */
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn init(&self) {
        self.window.init();
    }

    // Getters
    pub fn gobject(&self) -> &ApplicationWindow {
        &self.widget.gobject()
    }

    // Tools
    pub fn migrate(tx: &Transaction) -> Result<(), String> {
        // Migrate self components
        if let Err(e) = Database::init(&tx) {
            return Err(e.to_string());
        }

        // Delegate migration to childs
        /* @TODO
        Header::migrate(&tx)?; */
        Window::migrate(&tx)?;
        Widget::migrate(&tx)?;

        // Success
        Ok(())
    }
}
