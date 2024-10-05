mod action;
mod browser;
mod database;

use action::Action;
use browser::Browser;
use database::Database;

use gtk::{
    glib::ExitCode,
    prelude::{ActionExt, ApplicationExt, ApplicationExtManual, GtkApplicationExt, GtkWindowExt},
    Application,
};
use sqlite::Connection;

use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

const APPLICATION_ID: &str = "io.github.yggverse.Yoda";

pub struct App {
    // Actions
    // action_update: Arc<SimpleAction>,
    // Components
    // browser: Arc<Browser>,
    // Extras
    // database: Arc<Database>,
    // GTK
    application: Application,
}

impl App {
    // Construct
    pub fn new(
        profile_database_connection: Arc<RwLock<Connection>>,
        profile_path: PathBuf,
    ) -> Self {
        // Init database
        let database = {
            // Init writable database connection
            let mut connection = match profile_database_connection.write() {
                Ok(connection) => connection,
                Err(error) => todo!("{error}"),
            };

            // Init new transaction
            let transaction = match connection.transaction() {
                Ok(transaction) => transaction,
                Err(error) => todo!("{error}"),
            };

            // Init database structure
            match Database::init(&transaction) {
                Ok(database) => match transaction.commit() {
                    Ok(_) => Arc::new(database),
                    Err(error) => todo!("{error}"),
                },
                Err(error) => todo!("{error}"),
            }
        };

        // Init actions
        let action_tool_debug = Action::new("win", true);
        let action_tool_profile_directory = Action::new("win", true);
        let action_quit = Action::new("win", true);
        let action_update = Action::new("win", true);
        let action_tab_append = Action::new("win", true);
        let action_tab_close = Action::new("win", true);
        let action_tab_close_all = Action::new("win", true);
        let action_tab_page_navigation_base = Action::new("win", false);
        let action_tab_page_navigation_history_back = Action::new("win", false);
        let action_tab_page_navigation_history_forward = Action::new("win", false);
        let action_tab_page_navigation_reload = Action::new("win", true);
        let action_tab_pin = Action::new("win", true);

        // Init GTK
        let application = Application::builder()
            .application_id(APPLICATION_ID)
            .build();

        // Init accels
        application.set_accels_for_action(&action_tool_debug.detailed_name(), &["<Primary>i"]);
        application.set_accels_for_action(&action_update.detailed_name(), &["<Primary>u"]);
        application.set_accels_for_action(&action_quit.detailed_name(), &["<Primary>Escape"]);
        application.set_accels_for_action(&action_tab_append.detailed_name(), &["<Primary>t"]);
        application.set_accels_for_action(&action_tab_pin.detailed_name(), &["<Primary>p"]);
        application.set_accels_for_action(&action_tab_close.detailed_name(), &["<Primary>q"]);
        application.set_accels_for_action(
            &action_tab_page_navigation_base.detailed_name(),
            &["<Primary>h"],
        );
        application.set_accels_for_action(
            &action_tab_page_navigation_history_back.detailed_name(),
            &["<Primary>Left"],
        );
        application.set_accels_for_action(
            &action_tab_page_navigation_history_forward.detailed_name(),
            &["<Primary>Right"],
        );
        application.set_accels_for_action(
            &action_tab_page_navigation_reload.detailed_name(),
            &["<Primary>r"],
        );

        // Init components
        let browser = Arc::new(Browser::new(
            profile_database_connection.clone(),
            profile_path,
            action_tool_debug.simple(),
            action_tool_profile_directory.simple(),
            action_quit.simple(),
            action_update.simple(),
            action_tab_append.simple(),
            action_tab_close.simple(),
            action_tab_close_all.simple(),
            action_tab_page_navigation_base.simple(),
            action_tab_page_navigation_history_back.simple(),
            action_tab_page_navigation_history_forward.simple(),
            action_tab_page_navigation_reload.simple(),
            action_tab_pin.simple(),
        ));

        // Init events
        application.connect_activate({
            let action_update = action_update.simple();
            move |_| {
                // Make initial update
                action_update.activate(None);
            }
        });

        application.connect_startup({
            let browser = browser.clone();
            let database = database.clone();
            let profile_database_connection = profile_database_connection.clone();
            move |this| {
                // Init readable connection
                match profile_database_connection.read() {
                    Ok(connection) => {
                        // Create transaction
                        match connection.unchecked_transaction() {
                            Ok(transaction) => {
                                // Restore previous session from DB
                                match database.records(&transaction) {
                                    Ok(records) => {
                                        for record in records {
                                            browser.restore(&transaction, &record.id);
                                        }
                                    }
                                    Err(error) => todo!("{error}"),
                                }
                            }
                            Err(error) => todo!("{error}"),
                        }
                    }
                    Err(error) => todo!("{error}"),
                }

                // Assign browser window to this application
                browser.widget().set_application(Some(this));

                // Show main widget
                browser.widget().present();
            }
        });

        application.connect_shutdown({
            // let browser = browser.clone();
            let profile_database_connection = profile_database_connection.clone();
            let database = database.clone();
            move |_| {
                // Init writable connection
                match profile_database_connection.write() {
                    Ok(mut connection) => {
                        // Create transaction
                        match connection.transaction() {
                            Ok(transaction) => {
                                match database.records(&transaction) {
                                    Ok(records) => {
                                        // Cleanup previous session records
                                        for record in records {
                                            match database.delete(&transaction, &record.id) {
                                                Ok(_) => {
                                                    // Delegate clean action to childs
                                                    browser.clean(&transaction, &record.id);
                                                }
                                                Err(error) => todo!("{error}"),
                                            }
                                        }

                                        // Save current session to DB
                                        match database.add(&transaction) {
                                            Ok(_) => {
                                                // Delegate save action to childs
                                                browser.save(
                                                    &transaction,
                                                    &database.last_insert_id(&transaction),
                                                );
                                            }
                                            Err(error) => todo!("{error}"),
                                        }
                                    }
                                    Err(error) => todo!("{error}"),
                                }

                                // Confirm changes
                                if let Err(error) = transaction.commit() {
                                    todo!("{error}")
                                }
                            }
                            Err(error) => todo!("{error}"),
                        }
                    }
                    Err(error) => todo!("{error}"),
                }
            }
        });

        // Return activated App struct
        Self {
            // Actions (SimpleAction)
            // action_update: action_update.simple(),
            // Components
            // browser,
            // Extras
            // database,
            // GTK
            application,
        }
    }

    // Actions
    pub fn run(&self) -> ExitCode {
        self.application.run()
    }
}
