mod browser;

use std::fs;

use gtk::prelude::{ApplicationExt, ApplicationExtManual, GtkApplicationExt, GtkWindowExt};

use gtk::{glib, Application};

const APP_ID: &str = "io.github.yggverse.Yoda";

fn main() -> glib::ExitCode {
    // Init app
    let app = Application::builder().application_id(APP_ID).build();

    // Init accels
    app.set_accels_for_action("win.tab_append", &["<Primary>t"]);
    app.set_accels_for_action("win.tab_pin", &["<Primary>p"]);
    app.set_accels_for_action("win.tab_close", &["<Primary>q"]);
    app.set_accels_for_action("win.tab_page_base", &["<Primary>h"]);
    app.set_accels_for_action("win.tab_page_history_back", &["<Primary>Left"]);
    app.set_accels_for_action("win.tab_page_history_forward", &["<Primary>Right"]);
    app.set_accels_for_action("win.tab_page_reload", &["<Primary>r"]);
    app.set_accels_for_action("win.tab_page_bookmark", &["<Primary>b"]);
    app.set_accels_for_action("win.debug", &["<Primary>i"]);
    app.set_accels_for_action("win.quit", &["<Primary>Escape"]);

    // Create new window
    app.connect_activate({
        // Init profile directory
        let mut fs = glib::user_config_dir();

        fs.push(APP_ID);

        if let Err(e) = fs::create_dir_all(&fs) {
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
            browser::Browser::new(this, /*db.clone(),*/ 640, 480)
                .widget()
                .present();
        }
    });

    // Start
    app.run()
}
