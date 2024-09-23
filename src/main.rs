mod browser;

use std::fs;
use std::sync::Arc;

use gtk::prelude::{ApplicationExt, ApplicationExtManual, GtkApplicationExt, GtkWindowExt};

use gtk::{glib, Application};

const APP_ID: &str = "io.github.yggverse.Yoda";

fn main() -> glib::ExitCode {
    // Init app
    let app = Application::builder().application_id(APP_ID).build();

    // Init accels
    app.set_accels_for_action("win.tab_append", &["<Ctrl>t"]);
    app.set_accels_for_action("win.tab_pin", &["<Ctrl>p"]);
    app.set_accels_for_action("win.tab_close", &["<Ctrl>q"]);
    app.set_accels_for_action("win.debug", &["<Ctrl>i"]);
    app.set_accels_for_action("win.quit", &["<Ctrl>Escape"]);

    // Create new window
    app.connect_activate({
        // Init profile directory
        let mut fs = glib::user_config_dir();

        fs.push(APP_ID);

        if let Err(e) = fs::create_dir_all(&fs) {
            panic!("Failed to create profile directory: {e}")
        }

        // Init profile database
        let mut db = fs.clone();

        db.push("database.sqlite3");

        let db = match sqlite::open(db) {
            Ok(db) => Arc::new(db),
            Err(e) => panic!("Failed to connect profile database: {e}"),
        };

        move |this: &Application| {
            browser::Browser::new(this, db.clone(), 640, 480)
                .widget()
                .window()
                .present();
        }
    });

    // Start
    app.run()
}
