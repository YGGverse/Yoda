mod header;
mod main;

use gtk::{
    gio::ActionEntry,
    prelude::{ActionMapExtManual, GtkWindowExt},
    Application, ApplicationWindow,
};

use sqlite::Connection;

pub fn new(app: &Application, db: &Connection, width: i32, height: i32) -> ApplicationWindow {
    // Init browser window
    let browser = ApplicationWindow::builder()
        .default_width(width)
        .default_height(height)
        .application(app)
        .titlebar(&header::new())
        .child(&main::new())
        .build();

    // Init actions
    let action_debug = ActionEntry::builder("debug")
        .activate(|browser: &ApplicationWindow, _, _| {
            browser.emit_enable_debugging(true);
        })
        .build();

    let action_quit = ActionEntry::builder("quit")
        .activate(|browser: &ApplicationWindow, _, _| {
            browser.close();
        })
        .build();

    browser.add_action_entries([action_debug, action_quit]);

    // Done
    browser
}
