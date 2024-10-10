mod database;
mod widget;
mod window;

use database::Database;
use widget::Widget;
use window::Window;

use adw::ApplicationWindow;
use gtk::{
    gio::{AppInfo, AppLaunchContext, SimpleAction},
    prelude::{ActionMapExt, GtkWindowExt},
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
        action_tool_debug: Arc<SimpleAction>,
        action_tool_profile_directory: Arc<SimpleAction>,
        action_quit: Arc<SimpleAction>,
        action_update: Arc<SimpleAction>,
        action_tab_append: Arc<SimpleAction>,
        action_tab_close: Arc<SimpleAction>,
        action_tab_close_all: Arc<SimpleAction>,
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_tab_pin: Arc<SimpleAction>,
    ) -> Browser {
        let window = Arc::new(Window::new(
            action_tool_debug.clone(),
            action_tool_profile_directory.clone(),
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
        widget.gobject().add_action(action_tool_debug.as_ref());
        widget
            .gobject()
            .add_action(action_tool_profile_directory.as_ref());
        widget.gobject().add_action(action_quit.as_ref());
        widget.gobject().add_action(action_update.as_ref());
        widget.gobject().add_action(action_tab_append.as_ref());
        widget.gobject().add_action(action_tab_close.as_ref());
        widget.gobject().add_action(action_tab_close_all.as_ref());
        widget
            .gobject()
            .add_action(action_tab_page_navigation_base.as_ref());
        widget
            .gobject()
            .add_action(action_tab_page_navigation_history_back.as_ref());
        widget
            .gobject()
            .add_action(action_tab_page_navigation_history_forward.as_ref());
        widget
            .gobject()
            .add_action(action_tab_page_navigation_reload.as_ref());
        widget.gobject().add_action(action_tab_pin.as_ref());

        // Init events
        action_tool_debug.connect_activate({
            let widget = widget.clone();
            move |_, _| {
                widget.gobject().emit_enable_debugging(true);
            }
        });

        action_tool_profile_directory.connect_activate({
            move |_, _| {
                // @TODO [4_10] https://docs.gtk.org/gtk4/class.FileLauncher.html
                let _ = AppInfo::launch_default_for_uri(
                    &format!("file://{}", profile_path.to_string_lossy()),
                    Some(&AppLaunchContext::new()),
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
            move |_, _| {
                window.update();
            }
        });

        action_tab_append.connect_activate({
            let window = window.clone();
            move |_, _| {
                window.tab_append();
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
