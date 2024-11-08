mod browser;
mod database;

use browser::Browser;
use database::Database;

use adw::Application;
use gtk::{
    gio::SimpleAction,
    glib::{gformat, uuid_string_random, ExitCode},
    prelude::{
        ActionExt, ApplicationExt, ApplicationExtManual, GtkApplicationExt, GtkWindowExt,
        StaticVariantType, ToVariant,
    },
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
    // action_update: SimpleAction,
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
        // Init defaults
        let default_state = (-1).to_variant();

        // Init actions
        let action_about = SimpleAction::new(&uuid_string_random(), None);
        let action_debug = SimpleAction::new(&uuid_string_random(), None);
        let action_profile = SimpleAction::new(&uuid_string_random(), None);
        let action_quit = SimpleAction::new(&uuid_string_random(), None);
        let action_update =
            SimpleAction::new(&uuid_string_random(), Some(&String::static_variant_type()));
        let action_page_new = SimpleAction::new(&uuid_string_random(), None);
        let action_page_close =
            SimpleAction::new_stateful(&uuid_string_random(), None, &default_state);
        let action_page_close_all = SimpleAction::new(&uuid_string_random(), None);
        let action_page_home =
            SimpleAction::new_stateful(&uuid_string_random(), None, &default_state);
        let action_page_history_back =
            SimpleAction::new_stateful(&uuid_string_random(), None, &default_state);
        let action_page_history_forward =
            SimpleAction::new_stateful(&uuid_string_random(), None, &default_state);
        let action_page_reload =
            SimpleAction::new_stateful(&uuid_string_random(), None, &default_state);
        let action_page_pin =
            SimpleAction::new_stateful(&uuid_string_random(), None, &default_state);

        // Init GTK
        let gobject = Application::builder()
            .application_id(APPLICATION_ID)
            .build();

        // Init accels
        let accels_config = &[
            (
                gformat!("win.{}", action_page_reload.name()),
                &["<Primary>r"],
            ),
            (gformat!("win.{}", action_debug.name()), &["<Primary>i"]),
            (
                gformat!("win.{}", action_page_close.name()),
                &["<Primary>q"],
            ),
            (
                gformat!("win.{}", action_page_history_back.name()),
                &["<Primary>Left"],
            ),
            (
                gformat!("win.{}", action_page_history_forward.name()),
                &["<Primary>Right"],
            ),
            (gformat!("win.{}", action_page_home.name()), &["<Primary>h"]),
            (gformat!("win.{}", action_page_new.name()), &["<Primary>t"]),
            (gformat!("win.{}", action_page_pin.name()), &["<Primary>p"]),
            (gformat!("win.{}", action_quit.name()), &["<Primary>Escape"]),
            (gformat!("win.{}", action_update.name()), &["<Primary>u"]),
        ]; // @TODO config

        for (detailed_action_name, &accels) in accels_config {
            gobject.set_accels_for_action(detailed_action_name, &accels);
        }

        // Init components
        let browser = Arc::new(Browser::new(
            profile_path,
            action_about.clone(),
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

        // Init events
        gobject.connect_activate({
            let action_update = action_update.clone();
            move |_| {
                // Make initial update
                action_update.activate(Some(&"".to_variant())); // @TODO
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
                                        // Restore child components
                                        for record in records {
                                            if let Err(e) =
                                                browser.restore(&transaction, &record.id)
                                            {
                                                todo!("{e}")
                                            }
                                        }

                                        // Run initial features
                                        browser.init();
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
            match migrate(&transaction) {
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
}

// Tools
fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = Database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    browser::migrate(tx)?;

    // Success
    Ok(())
}
