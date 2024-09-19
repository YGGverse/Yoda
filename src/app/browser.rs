#[path = "browser/header.rs"] mod header;
#[path = "browser/main.rs"] mod main;

use gtk::{
    Application,
    ApplicationWindow,
    gio::ActionEntry,
    prelude::{
        ActionMapExtManual,
        GtkWindowExt
    }
};

pub fn new(
    app: &Application,
    width: i32,
    height: i32
) -> ApplicationWindow
{
    // Init browser window
    let browser = ApplicationWindow::builder()

        // Tuneup
        .default_width(
            width
        )

        .default_height(
            height
        )

        // Relate
        .application(
            app
        )

        // Init components
        .titlebar(
            &header::new()
        )

        .child(
            &main::new()
        )

        // Make
        .build();

        // Init actions
        let action_debug = ActionEntry::builder("debug")

            .activate(
                |browser: &ApplicationWindow, _, _|
                {
                    browser.emit_enable_debugging(
                        true
                    );
                }
            )

            .build();

        let action_quit = ActionEntry::builder("quit")

            .activate(
                |browser: &ApplicationWindow, _, _|
                {
                    browser.close();
                }
            )

            .build();

    browser.add_action_entries(
        [
            action_debug,
            action_quit
        ]
    );

    // Done
    browser
}