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
use sqlite::{Connection, Transaction};

use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

const APPLICATION_ID: &str = "io.github.yggverse.Yoda";

pub struct App {
    profile_database_connection: Arc<RwLock<Connection>>,
    // database: Arc<Database>,
    // Actions
    // action_update: Arc<SimpleAction>,
    // Components
    // browser: Arc<Browser>,
    // GTK
    gobject: Application,
}

impl App {
    // Construct
    pub fn new(
        profile_database_connection: Arc<RwLock<Connection>>,
        profile_path: PathBuf,
    ) -> Self {
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
        let gobject = Application::builder()
            .application_id(APPLICATION_ID)
            .build();

        // Init accels
        gobject.set_accels_for_action(&action_tool_debug.detailed_name(), &["<Primary>i"]);
        gobject.set_accels_for_action(&action_update.detailed_name(), &["<Primary>u"]);
        gobject.set_accels_for_action(&action_quit.detailed_name(), &["<Primary>Escape"]);
        gobject.set_accels_for_action(&action_tab_append.detailed_name(), &["<Primary>t"]);
        gobject.set_accels_for_action(&action_tab_pin.detailed_name(), &["<Primary>p"]);
        gobject.set_accels_for_action(&action_tab_close.detailed_name(), &["<Primary>q"]);
        gobject.set_accels_for_action(
            &action_tab_page_navigation_base.detailed_name(),
            &["<Primary>h"],
        );
        gobject.set_accels_for_action(
            &action_tab_page_navigation_history_back.detailed_name(),
            &["<Primary>Left"],
        );
        gobject.set_accels_for_action(
            &action_tab_page_navigation_history_forward.detailed_name(),
            &["<Primary>Right"],
        );
        gobject.set_accels_for_action(
            &action_tab_page_navigation_reload.detailed_name(),
            &["<Primary>r"],
        );

        // Init components
        let browser = Arc::new(Browser::new(
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
        gobject.connect_activate({
            let action_update = action_update.simple();
            move |_| {
                // Make initial update
                action_update.activate(None);
            }
        });

        gobject.connect_startup({
            let browser = browser.clone();
            let profile_database_connection = profile_database_connection.clone();
            move |this| {
                // Init readable connection
                match profile_database_connection.read() {
                    Ok(connection) => {
                        // Create transaction
                        match connection.unchecked_transaction() {
                            Ok(transaction) => {
                                // Restore previous session from DB
                                match Database::records(&transaction) {
                                    Ok(records) => {
                                        for record in records {
                                            if let Err(e) =
                                                browser.restore(&transaction, &record.id)
                                            {
                                                todo!("{e}")
                                            }
                                        }
                                    }
                                    Err(e) => todo!("{e}"),
                                }
                            }
                            Err(e) => todo!("{e}"),
                        }
                    }
                    Err(e) => todo!("{e}"),
                }

                // Assign browser window to this application
                browser.gobject().set_application(Some(this));

                // Show main widget
                browser.gobject().present();
            }
        });

        gobject.connect_shutdown({
            // let browser = browser.clone();
            let profile_database_connection = profile_database_connection.clone();
            let browser = browser.clone();
            move |_| {
                // Init writable connection
                match profile_database_connection.write() {
                    Ok(mut connection) => {
                        // Create transaction
                        match connection.transaction() {
                            Ok(transaction) => {
                                match Database::records(&transaction) {
                                    Ok(records) => {
                                        // Cleanup previous session records
                                        for record in records {
                                            match Database::delete(&transaction, &record.id) {
                                                Ok(_) => {
                                                    // Delegate clean action to childs
                                                    if let Err(e) =
                                                        browser.clean(&transaction, &record.id)
                                                    {
                                                        todo!("{e}")
                                                    }
                                                }
                                                Err(e) => todo!("{e}"),
                                            }
                                        }

                                        // Save current session to DB
                                        match Database::add(&transaction) {
                                            Ok(_) => {
                                                // Delegate save action to childs
                                                if let Err(e) = browser.save(
                                                    &transaction,
                                                    &Database::last_insert_id(&transaction),
                                                ) {
                                                    todo!("{e}")
                                                }
                                            }
                                            Err(e) => todo!("{e}"),
                                        }
                                    }
                                    Err(e) => todo!("{e}"),
                                }

                                // Confirm changes
                                if let Err(e) = transaction.commit() {
                                    todo!("{e}")
                                }
                            }
                            Err(e) => todo!("{e}"),
                        }
                    }
                    Err(e) => todo!("{e}"),
                }
            }
        });

        // Return activated App struct
        Self {
            // database,
            profile_database_connection,
            // Actions (SimpleAction)
            // action_update: action_update.simple(),
            // Components
            // browser,
            // GTK
            gobject,
        }
    }

    // Actions
    pub fn run(&self) -> ExitCode {
        // Begin database migration @TODO
        {
            // Init writable connection
            let mut connection = match self.profile_database_connection.try_write() {
                Ok(connection) => connection,
                Err(e) => todo!("{e}"),
            };

            // Init new transaction
            let transaction = match connection.transaction() {
                Ok(transaction) => transaction,
                Err(e) => todo!("{e}"),
            };

            // Begin migration
            match App::migrate(&transaction) {
                Ok(_) => {
                    // Confirm changes
                    match transaction.commit() {
                        Ok(_) => {} // @TODO
                        Err(e) => todo!("{e}"),
                    }
                }
                Err(e) => todo!("{e}"),
            }
        } // unlock database

        // Start application
        self.gobject.run()
    }

    // Tools
    fn migrate(tx: &Transaction) -> Result<(), String> {
        // Migrate self components
        if let Err(e) = Database::init(&tx) {
            return Err(e.to_string());
        }

        // Delegate migration to childs
        Browser::migrate(&tx)?;

        // Success
        Ok(())
    }
}
