mod action;
mod browser;
mod database;

use action::Action;
use browser::Browser;
use database::Database;

use gtk::{
    glib::{user_config_dir, ExitCode},
    prelude::{ActionExt, ApplicationExt, ApplicationExtManual, GtkApplicationExt, GtkWindowExt},
    Application,
};

use std::{fs::create_dir_all, sync::Arc};

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
    pub fn new() -> Self {
        // Init profile directory path
        let mut profile_path = user_config_dir();

        profile_path.push(APPLICATION_ID);

        if let Err(e) = create_dir_all(&profile_path) {
            panic!("Failed to create profile directory: {e}")
        }

        // Init profile database path
        let mut database_path = profile_path.clone();

        database_path.push("database.sqlite3");

        let connection = match sqlite::open(database_path) {
            Ok(connection) => Arc::new(connection),
            Err(e) => panic!("Failed to connect profile database: {e}"),
        };

        // Init database model
        let database = Arc::new(Database::init(connection, env!("CARGO_PKG_VERSION")));

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
            let database = database.clone();
            move |application| {
                // Restore previous session
                database.restore();

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
}
