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

use std::{path::PathBuf, sync::Arc};

const APPLICATION_ID: &str = "io.github.yggverse.Yoda";

pub struct App {
    // Actions
    // action_update: Arc<SimpleAction>,
    // Components
    // browser: Arc<Browser>,
    // Extras
    // database: Arc<Database>,
    // GTK
    app: Application,
}

impl App {
    // Construct
    pub fn new(profile_database_connection: Arc<Connection>, profile_path: PathBuf) -> Self {
        // Init database
        let database = match Database::init(profile_database_connection.clone()) {
            Ok(database) => Arc::new(database),
            Err(error) => panic!("{error}"), // @TODO
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
        let app = Application::builder()
            .application_id(APPLICATION_ID)
            .build();

        // Init accels
        app.set_accels_for_action(&action_tool_debug.detailed_name(), &["<Primary>i"]);
        app.set_accels_for_action(&action_update.detailed_name(), &["<Primary>u"]);
        app.set_accels_for_action(&action_quit.detailed_name(), &["<Primary>Escape"]);
        app.set_accels_for_action(&action_tab_append.detailed_name(), &["<Primary>t"]);
        app.set_accels_for_action(&action_tab_pin.detailed_name(), &["<Primary>p"]);
        app.set_accels_for_action(&action_tab_close.detailed_name(), &["<Primary>q"]);
        app.set_accels_for_action(
            &action_tab_page_navigation_base.detailed_name(),
            &["<Primary>h"],
        );
        app.set_accels_for_action(
            &action_tab_page_navigation_history_back.detailed_name(),
            &["<Primary>Left"],
        );
        app.set_accels_for_action(
            &action_tab_page_navigation_history_forward.detailed_name(),
            &["<Primary>Right"],
        );
        app.set_accels_for_action(
            &action_tab_page_navigation_reload.detailed_name(),
            &["<Primary>r"],
        );

        // Init components
        let browser = Arc::new(Browser::new(
            profile_database_connection,
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
        app.connect_activate({
            let action_update = action_update.simple();
            move |_| {
                // Make initial update
                action_update.activate(None);
            }
        });

        app.connect_startup({
            let browser = browser.clone();
            let database = database.clone();
            move |this| {
                // Restore previous session from DB
                match database.records() {
                    Ok(records) => {
                        for record in records {
                            browser.restore(record.id);
                        }
                    }
                    Err(error) => panic!("{error}"), // @TODO
                }

                // Assign browser window to this application
                browser.widget().set_application(Some(this));

                // Show main widget
                browser.widget().present();
            }
        });

        app.connect_shutdown({
            // let browser = browser.clone();
            let database = database.clone();
            move |_| {
                // @TODO transaction?
                match database.records() {
                    Ok(records) => {
                        // Cleanup previous session records
                        for record in records {
                            match database.delete(record.id) {
                                Ok(_) => {
                                    // Delegate clean action to childs
                                    browser.clean(record.id);
                                }
                                Err(error) => panic!("{error}"), // @TODO
                            }
                        }

                        // Save current session to DB
                        match database.add() {
                            Ok(_) => {
                                // Delegate save action to childs
                                browser.save(database.last_insert_id());
                            }
                            Err(error) => panic!("{error}"), // @TODO
                        }
                    }
                    Err(error) => panic!("{error}"), // @TODO
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
            app,
        }
    }

    // Actions
    pub fn run(&self) -> ExitCode {
        self.app.run()
    }
}
