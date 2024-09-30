mod browser;

use browser::Browser;

use gtk::{
    gio::SimpleAction,
    glib::{user_config_dir, ExitCode},
    prelude::{ActionExt, ApplicationExt, ApplicationExtManual, GtkApplicationExt, GtkWindowExt},
    Application,
};

use std::{fs::create_dir_all, sync::Arc};

const APP_ID: &str = "io.github.yggverse.Yoda";

fn main() -> ExitCode {
    // Init app
    let app = Application::builder().application_id(APP_ID).build();

    // Init actions
    let action_debug = Arc::new(SimpleAction::new("debug", None));
    let action_quit = Arc::new(SimpleAction::new("quit", None));
    let action_update = Arc::new(SimpleAction::new("update", None));

    let action_tab_append = Arc::new(SimpleAction::new("tab_append", None));
    let action_tab_close = Arc::new(SimpleAction::new("tab_close", None));
    let action_tab_close_all = Arc::new(SimpleAction::new("tab_close_all", None));
    let action_tab_page_navigation_base =
        Arc::new(SimpleAction::new("tab_page_navigation_base", None));
    let action_tab_page_navigation_history_back =
        Arc::new(SimpleAction::new("tab_page_navigation_history_back", None));
    let action_tab_page_navigation_history_forward = Arc::new(SimpleAction::new(
        "tab_page_navigation_history_forward",
        None,
    ));
    let action_tab_page_navigation_reload =
        Arc::new(SimpleAction::new("tab_page_navigation_reload", None));
    let action_tab_pin = Arc::new(SimpleAction::new("tab_pin", None));

    // Init accels
    app.set_accels_for_action("win.debug", &["<Primary>i"]);
    app.set_accels_for_action("win.update", &["<Primary>u"]);
    app.set_accels_for_action("win.quit", &["<Primary>Escape"]);

    app.set_accels_for_action("win.tab_append", &["<Primary>t"]);
    app.set_accels_for_action("win.tab_pin", &["<Primary>p"]);
    app.set_accels_for_action("win.tab_close", &["<Primary>q"]);
    app.set_accels_for_action("win.tab_page_navigation_base", &["<Primary>h"]);
    app.set_accels_for_action("win.tab_page_navigation_history_back", &["<Primary>Left"]);
    app.set_accels_for_action(
        "win.tab_page_navigation_history_forward",
        &["<Primary>Right"],
    );
    app.set_accels_for_action("win.tab_page_navigation_reload", &["<Primary>r"]);
    //app.set_accels_for_action("win.tab_page_bookmark", &["<Primary>b"]);

    // Create new window
    app.connect_activate({
        // Init profile directory
        let mut fs = user_config_dir();

        fs.push(APP_ID);

        if let Err(e) = create_dir_all(&fs) {
            panic!("Failed to create profile directory: {e}")
        }

        // Init profile database
        /* @TODO
        let mut db = fs.clone();

        db.push("database.sqlite3");

        let db = match sqlite::open(db) {
            Ok(db) => Arc::new(db),
            Err(e) => panic!("Failed to connect profile database: {e}"),
        };*/

        move |this: &Application| {
            Browser::new(
                this,
                /*db.clone(),*/
                action_debug.clone(),
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
            )
            .widget()
            .present();

            // Make initial update
            action_update.activate(None);
        }
    });

    // Start
    app.run()
}
