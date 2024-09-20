mod browser;

use gtk::prelude::{ApplicationExt, ApplicationExtManual, GtkApplicationExt, GtkWindowExt};

use gtk::{glib, Application};

fn main() -> glib::ExitCode {
    // Init app
    let app = Application::builder()
        .application_id("io.github.yggverse.Yoda.app")
        .build();

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
