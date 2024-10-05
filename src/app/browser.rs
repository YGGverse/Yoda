mod database;
mod header;
mod widget;
mod window;

use database::Database;
use header::Header;
use sqlite::Transaction;
use widget::Widget;
use window::Window;

use gtk::{
    gio::{AppInfo, AppLaunchContext, SimpleAction},
    prelude::{ActionMapExt, GtkWindowExt},
    ApplicationWindow,
};
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

pub struct Browser {
    // Extras
    database: Arc<Database>,
    // Components
    // header: Arc<Header>,
    // window: Arc<Window>,
    widget: Arc<Widget>,
}

impl Browser {
    // Construct
    pub fn new(
        // Extras
        profile_database_connection: Arc<RwLock<sqlite::Connection>>,
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

        // Init components
        let header = Arc::new(Header::new(
            action_tool_debug.clone(),
            action_tool_profile_directory.clone(),
            action_quit.clone(),
            action_tab_append.clone(),
            action_tab_close.clone(),
            action_tab_close_all.clone(),
            action_tab_page_navigation_base.clone(),
            action_tab_page_navigation_history_back.clone(),
            action_tab_page_navigation_history_forward.clone(),
            action_tab_page_navigation_reload.clone(),
            action_tab_pin.clone(),
        ));

        let main = Arc::new(Window::new(
            action_tab_page_navigation_base.clone(),
            action_tab_page_navigation_history_back.clone(),
            action_tab_page_navigation_history_forward.clone(),
            action_tab_page_navigation_reload.clone(),
            action_update.clone(),
        ));

        // Init widget
        let widget = Arc::new(Widget::new(
            profile_database_connection.clone(),
            header.widget(),
            main.widget(),
        ));

        // Assign actions
        widget
            .application_window()
            .add_action(action_tool_debug.as_ref());

        widget
            .application_window()
            .add_action(action_tool_profile_directory.as_ref());

        widget.application_window().add_action(action_quit.as_ref());

        widget
            .application_window()
            .add_action(action_update.as_ref());

        widget
            .application_window()
            .add_action(action_tab_append.as_ref());

        widget
            .application_window()
            .add_action(action_tab_close.as_ref());

        widget
            .application_window()
            .add_action(action_tab_close_all.as_ref());

        widget
            .application_window()
            .add_action(action_tab_page_navigation_base.as_ref());

        widget
            .application_window()
            .add_action(action_tab_page_navigation_history_back.as_ref());

        widget
            .application_window()
            .add_action(action_tab_page_navigation_history_forward.as_ref());

        widget
            .application_window()
            .add_action(action_tab_page_navigation_reload.as_ref());

        widget
            .application_window()
            .add_action(action_tab_pin.as_ref());

        // Init events
        action_tool_debug.connect_activate({
            let widget = widget.clone();
            move |_, _| {
                widget.application_window().emit_enable_debugging(true);
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
                widget.application_window().close();
            }
        });

        action_update.connect_activate({
            let header = header.clone();
            let main = main.clone();
            move |_, _| {
                main.update();
                header.update(main.tab_page_title(), main.tab_page_description());
            }
        });

        action_tab_append.connect_activate({
            let main = main.clone();
            move |_, _| {
                main.tab_append(None);
            }
        });

        action_tab_close.connect_activate({
            let main = main.clone();
            move |_, _| {
                main.tab_close();
            }
        });

        action_tab_close_all.connect_activate({
            let main = main.clone();
            move |_, _| {
                main.tab_close_all();
            }
        });

        action_tab_page_navigation_base.connect_activate({
            let main = main.clone();
            move |_, _| {
                main.tab_page_navigation_base();
            }
        });

        action_tab_page_navigation_history_back.connect_activate({
            let main = main.clone();
            move |_, _| {
                main.tab_page_navigation_history_back();
            }
        });

        action_tab_page_navigation_history_forward.connect_activate({
            let main = main.clone();
            move |_, _| {
                main.tab_page_navigation_history_forward();
            }
        });

        action_tab_page_navigation_reload.connect_activate({
            let main = main.clone();
            move |_, _| {
                main.tab_page_navigation_reload();
            }
        });

        action_tab_pin.connect_activate({
            let main = main.clone();
            move |_, _| {
                main.tab_pin();
            }
        });

        // Return new activated Browser struct
        Self {
            database,
            widget,
            // header,
            // main,
        }
    }

    // Actions
    pub fn clean(&self, tx: &Transaction, app_id: &i64) {
        match self.database.records(tx, app_id) {
            Ok(records) => {
                for record in records {
                    match self.database.delete(tx, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to childs
                            // @TODO
                            // self.header.clean(record.id);
                            // self.main.clean(record.id);

                            self.widget.clean(tx, &record.id);
                        }
                        Err(e) => todo!("{e}"),
                    }
                }
            }
            Err(e) => todo!("{e}"),
        }
    }

    pub fn restore(&self, tx: &Transaction, app_id: &i64) {
        match self.database.records(tx, app_id) {
            Ok(records) => {
                for record in records {
                    // Delegate restore action to childs
                    // @TODO
                    // self.header.restore(record.id);
                    // self.window.restore(record.id);

                    self.widget.restore(tx, &record.id);
                }
            }
            Err(e) => todo!("{e}"),
        }
    }

    pub fn save(&self, tx: &Transaction, app_id: &i64) {
        match self.database.add(tx, app_id) {
            Ok(_) => {
                // Delegate save action to childs
                let id = self.database.last_insert_id(tx);

                // @TODO
                // self.header.save(id);
                // self.window.save(id);

                self.widget.save(tx, &id);
            }
            Err(e) => todo!("{e}"),
        }
    }

    // Getters
    pub fn widget(&self) -> &ApplicationWindow {
        &self.widget.application_window()
    }
}
