#[path = "browser/header.rs"] mod header;
#[path = "browser/main.rs"] mod main;

use gtk::{Application, ApplicationWindow};

pub fn new(app: &Application) -> ApplicationWindow
{
    return ApplicationWindow::builder().application(app)
                                       .default_width(640)
                                       .default_height(480)
                                       .titlebar(&header::new())
                                       .child(&main::new())
                                       .build();
}