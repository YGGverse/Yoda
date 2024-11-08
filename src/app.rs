mod browser;
mod database;

use browser::Browser;
use database::Database;

use crate::profile::Profile;
use adw::Application;
use gtk::{
    gio::SimpleAction,
    glib::{gformat, uuid_string_random, ExitCode},
    prelude::{
        ActionExt, ApplicationExt, ApplicationExtManual, GtkApplicationExt, GtkWindowExt, ToVariant,
    },
};
use sqlite::Transaction;
use std::rc::Rc;

const APPLICATION_ID: &str = "io.github.yggverse.Yoda";

pub struct App {
    profile: Rc<Profile>,
    gobject: Application,
}

impl App {
    // Construct
    pub fn new() -> Self {
        // Init profile
        let profile = Rc::new(Profile::new());

        // @TODO
        let default_state = (-1).to_variant();

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

        // Init components
        let browser = Rc::new(Browser::new(
            profile.clone(),
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
            let update = browser.action().update().clone();
            move |_| {
                // Make initial update
                update.activate(Some(&"".to_variant())); // @TODO
            }
        });

        gobject.connect_startup({
            let browser = browser.clone();
            let profile = profile.clone();
            move |this| {
                // Init readable connection
                match profile.database().connection().read() {
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
            let browser = browser.clone();
            let profile = profile.clone();
            move |_| {
                // Init writable connection
                match profile.database().connection().write() {
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

        // Init accels
        let accels_config = &[
            // Browser actions
            (
                gformat!("win.{}", browser.action().debug().name()),
                &["<Primary>i"],
            ),
            (
                gformat!("win.{}", browser.action().quit().name()),
                &["<Primary>Escape"],
            ),
            (
                gformat!("win.{}", browser.action().update().name()),
                &["<Primary>u"],
            ),
            // Other
            (
                gformat!("win.{}", action_page_reload.name()),
                &["<Primary>r"],
            ),
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
        ]; // @TODO config

        for (detailed_action_name, &accels) in accels_config {
            gobject.set_accels_for_action(detailed_action_name, &accels);
        }

        // Return activated App struct
        Self { profile, gobject }
    }

    // Actions
    pub fn run(&self) -> ExitCode {
        // Begin database migration @TODO
        {
            // Init writable connection
            let mut connection = match self.profile.database().connection().try_write() {
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
