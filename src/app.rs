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

use std::sync::Arc;

const APPLICATION_ID: &str = "io.github.yggverse.Yoda";

pub struct App {
    // GTK
    app: Application,
    // Components
    //browser: Arc<Browser>,
    database: Arc<Database>,
}

impl App {
    // Construct
    pub fn new(profile_database_connection: Arc<Connection>) -> Self {
        // Init app database model
        let database = Arc::new(Database::init(profile_database_connection));

        // Init actions
        let action_debug = Action::new("win", true);
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
        app.set_accels_for_action(&action_debug.detailed_name(), &["<Primary>i"]);
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

        // Init events
        app.connect_activate({
            // let database = database.clone();
            move |application| {
                // Restore previous session
                // @TODO

                // Init components
                let browser = Arc::new(Browser::new(
                    &application,
                    /*db.clone(),*/
                    action_debug.simple(),
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

                // Show main widget
                browser.widget().present();

                // Make initial update
                action_update.simple().activate(None);
            }
        });

        // Return activated struct
        Self { app, database }
    }

    // Actions
    pub fn run(&self) -> ExitCode {
        self.app.run()
    }

    pub fn save(&self) {
        // Cleanup previous record
        match self.database.clean() {
            Ok(_) => {
                // Delegate clean action to children components
                // self.browser.clean(app_id) @TODO
                // ..

                // Create new record
                match self.database.add() {
                    Ok(_) => {
                        // let app_id = self.database.last_insert_id();

                        // Delegate save action to children components
                        // self.browser.save(app_id) @TODO
                        // ..
                    }
                    Err(error) => panic!("{error}"), // @TODO
                }
            }
            Err(error) => panic!("{error}"), // @TODO
        }
    }
}
