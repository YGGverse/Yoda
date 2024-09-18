#[path = "browser/header.rs"] mod header;
#[path = "browser/main.rs"] mod main;

use gtk::{Application, ApplicationWindow};

pub fn new(
    app: &Application,
    width: i32,
    height: i32
) -> ApplicationWindow
{
    return ApplicationWindow::builder()

        // Relate
        .application(
            app
        )

        // Tuneup
        .default_width(
            width
        )

        .default_height(
            height
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
}