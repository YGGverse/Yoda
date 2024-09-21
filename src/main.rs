mod browser;

use std::fs;

use gtk::prelude::{ApplicationExt, ApplicationExtManual, GtkApplicationExt, GtkWindowExt};

use gtk::{glib, Application};

fn main() -> glib::ExitCode {
    // Init meta
    const APP_ID: &str = "io.github.yggverse.Yoda";

    // Init config location
    let mut config = gtk::glib::user_config_dir();

    config.push(APP_ID);

    if let Err(explain) = fs::create_dir_all(config) {
        panic!("Failed to create profile directory: {explain}")
    }

    // Init app
    let app = Application::builder().application_id(APP_ID).build();

    // Init accels
    app.set_accels_for_action("win.tab_append", &["<Ctrl>t"]);
    app.set_accels_for_action("win.tab_close", &["<Ctrl>q"]);
    app.set_accels_for_action("win.debug", &["<Ctrl>i"]);
    app.set_accels_for_action("win.quit", &["<Ctrl>Escape"]);

    // Create new window
    app.connect_activate(|app| {
        browser::new(app, 640, 480).present();
    });

    // Start
    app.run()
}
