mod browser;
mod database;

use browser::Browser;

use crate::profile::Profile;
use adw::Application;
use gtk::{
    glib::ExitCode,
    prelude::{ActionExt, ApplicationExt, ApplicationExtManual, GtkApplicationExt},
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
    pub fn new(profile: Rc<Profile>) -> Self {
        // Init GTK
        let gobject = Application::builder()
            .application_id(APPLICATION_ID)
            .build();

        // Init components
        let browser = Rc::new(Browser::new(profile.clone()));

        // Init events
        gobject.connect_activate({
            let browser = browser.clone();
            move |_| browser.update()
        });

        gobject.connect_startup({
            let browser = browser.clone();
            let profile = profile.clone();
            move |this| {
                // Init readable connection
                match profile.database.connection.read() {
                    Ok(connection) => {
                        // Create transaction
                        match connection.unchecked_transaction() {
                            Ok(transaction) => {
                                // Restore previous session from DB
                                match database::select(&transaction) {
                                    Ok(records) => {
                                        // Restore child components
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

                // Run initial features, show widget
                browser.init(Some(this)).present();
            }
        });

        gobject.connect_shutdown({
            let browser = browser.clone();
            let profile = profile.clone();
            move |_| {
                // Init writable connection
                match profile.database.connection.write() {
                    Ok(mut connection) => {
                        // Create transaction
                        match connection.transaction() {
                            Ok(transaction) => {
                                match database::select(&transaction) {
                                    Ok(records) => {
                                        // Cleanup previous session records
                                        for record in records {
                                            match database::delete(&transaction, &record.id) {
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
                                        match database::insert(&transaction) {
                                            Ok(_) => {
                                                // Delegate save action to childs
                                                if let Err(e) = browser.save(
                                                    &transaction,
                                                    &database::last_insert_id(&transaction),
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
        for (detailed_action_name, &accels) in &[
            // Browser actions
            (
                format!(
                    "{}.{}",
                    browser.action.id,
                    browser.action.close.gobject.name()
                ),
                &["<Primary>Escape"],
            ),
            (
                format!(
                    "{}.{}",
                    browser.action.id,
                    browser.action.debug.gobject.name()
                ),
                &["<Primary>i"],
            ),
            (
                format!(
                    "{}.{}",
                    browser.action.id,
                    browser.action.update.gobject.name()
                ),
                &["<Primary>u"],
            ),
            (
                format!(
                    "{}.{}",
                    browser.action.id,
                    browser.action.focus.gobject.name()
                ),
                &["Escape"],
            ),
            // Tab actions
            (
                format!(
                    "{}.{}",
                    browser.window.action.id,
                    browser.window.action.append.gobject.name()
                ),
                &["<Primary>t"],
            ),
            (
                format!(
                    "{}.{}",
                    browser.window.action.id,
                    browser.window.action.bookmark.gobject.name()
                ),
                &["<Primary>b"],
            ),
            (
                format!(
                    "{}.{}",
                    browser.window.action.id,
                    browser.window.action.pin.gobject.name()
                ),
                &["<Primary>p"],
            ),
            (
                format!(
                    "{}.{}",
                    browser.window.action.id,
                    browser.window.action.reload.gobject.name()
                ),
                &["<Primary>r"],
            ),
            (
                format!(
                    "{}.{}",
                    browser.window.action.id,
                    browser.window.action.save_as.gobject.name()
                ),
                &["<Primary>s"],
            ),
            (
                format!(
                    "{}.{}",
                    browser.window.action.id,
                    browser.window.action.source.gobject.name()
                ),
                &["<Primary>u"],
            ),
            (
                format!(
                    "{}.{}",
                    browser.window.action.id,
                    browser.window.action.home.gobject.name()
                ),
                &["<Primary>h"],
            ),
            (
                format!(
                    "{}.{}",
                    browser.window.action.id,
                    browser.window.action.history_back.gobject.name()
                ),
                &["<Primary>Left"],
            ),
            (
                format!(
                    "{}.{}",
                    browser.window.action.id,
                    browser.window.action.history_forward.gobject.name()
                ),
                &["<Primary>Right"],
            ),
            (
                format!(
                    "{}.{}",
                    browser.window.action.id,
                    browser.window.action.close.gobject.name()
                ),
                &["<Primary>q"],
            ),
        ] {
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
            let mut connection = match self.profile.database.connection.write() {
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
    if let Err(e) = database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    browser::migrate(tx)?;

    // Success
    Ok(())
}
