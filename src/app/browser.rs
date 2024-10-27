mod database;
mod widget;
mod window;

use database::Database;
use widget::Widget;
use window::Window;

use adw::ApplicationWindow;
use gtk::{
    gio::{Cancellable, File, SimpleAction},
    prelude::{ActionMapExt, GtkWindowExt},
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
        action_tool_debug: SimpleAction,
        action_tool_profile: SimpleAction,
        action_quit: SimpleAction,
        action_update: SimpleAction,
        action_tab_append: SimpleAction,
        action_tab_close: SimpleAction,
        action_tab_close_all: SimpleAction,
        action_tab_page_navigation_base: SimpleAction,
        action_tab_page_navigation_history_back: SimpleAction,
        action_tab_page_navigation_history_forward: SimpleAction,
        action_tab_page_navigation_reload: SimpleAction,
        action_tab_pin: SimpleAction,
    ) -> Browser {
        let window = Arc::new(Window::new(
            action_tool_debug.clone(),
            action_tool_profile.clone(),
            action_quit.clone(),
            action_update.clone(),
            action_tab_append.clone(),
            action_tab_close.clone(),
            action_tab_close_all.clone(),
            action_tab_page_navigation_base.clone(),
            action_tab_page_navigation_history_back.clone(),
            action_tab_page_navigation_history_forward.clone(),
            action_tab_page_navigation_reload.clone(),
            action_tab_pin.clone(),
        ));

        // Init widget
        let widget = Arc::new(Widget::new(window.gobject()));

        // Assign actions
        widget.gobject().add_action(&action_tool_debug);
        widget.gobject().add_action(&action_tool_profile);
        widget.gobject().add_action(&action_quit);
        widget.gobject().add_action(&action_update);
        widget.gobject().add_action(&action_tab_append);
        widget.gobject().add_action(&action_tab_close);
        widget.gobject().add_action(&action_tab_close_all);
        widget
            .gobject()
            .add_action(&action_tab_page_navigation_base);
        widget
            .gobject()
            .add_action(&action_tab_page_navigation_history_back);
        widget
            .gobject()
            .add_action(&action_tab_page_navigation_history_forward);
        widget
            .gobject()
            .add_action(&action_tab_page_navigation_reload);
        widget.gobject().add_action(&action_tab_pin);

        // Init events
        action_tool_debug.connect_activate({
            let widget = widget.clone();
            move |_, _| {
                widget.gobject().emit_enable_debugging(true);
            }
        });

        action_tool_profile.connect_activate({
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

        action_tab_append.connect_activate({
            let window = window.clone();
            move |_, _| {
                window.tab_append(None);
            }
        });

        action_tab_close.connect_activate({
            let window = window.clone();
            move |_, _| {
                window.tab_close();
            }
        });

        action_tab_close_all.connect_activate({
            let window = window.clone();
            move |_, _| {
                window.tab_close_all();
            }
        });

        action_tab_page_navigation_base.connect_activate({
            let window = window.clone();
            move |_, _| {
                window.tab_page_navigation_base();
            }
        });

        action_tab_page_navigation_history_back.connect_activate({
            let window = window.clone();
            move |_, _| {
                window.tab_page_navigation_history_back();
            }
        });

        action_tab_page_navigation_history_forward.connect_activate({
            let window = window.clone();
            move |_, _| {
                window.tab_page_navigation_history_forward();
            }
        });

        action_tab_page_navigation_reload.connect_activate({
            let window = window.clone();
            move |_, _| {
                window.tab_page_navigation_reload();
            }
        });

        action_tab_pin.connect_activate({
            let window = window.clone();
            move |_, _| {
                window.tab_pin();
            }
        });

        // Return new activated Browser struct
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
