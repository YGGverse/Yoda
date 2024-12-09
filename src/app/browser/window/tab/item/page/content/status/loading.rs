use adw::StatusPage;
use gtk::{
    glib::{timeout_add_local, ControlFlow},
    prelude::WidgetExt,
    Spinner, // use adw::Spinner; @TODO adw 1.6 / ubuntu 24.10+
};
use std::time::Duration;

const SPINNER_SIZE: i32 = 32; // 16-64
const DEFAULT_TITLE: &str = "Loading..";

/// Create new default preset for loading
/// [StatusPage](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.StatusPage.html)
pub fn new_gobject(show_with_delay: Option<Duration>) -> StatusPage {
    // Init spinner component
    let spinner = Spinner::builder()
        .width_request(SPINNER_SIZE)
        .height_request(SPINNER_SIZE)
        .build();

    // Init main widget
    let status_page = StatusPage::builder()
        .child(&spinner)
        .title(DEFAULT_TITLE)
        .build();

    // Apply optional delay
    match show_with_delay {
        Some(duration) => {
            timeout_add_local(duration, {
                let status_page = status_page.clone();
                move || {
                    status_page.set_visible(true);
                    spinner.start();
                    ControlFlow::Break
                }
            });
        }
        None => {
            status_page.set_visible(true);
            spinner.start();
        }
    }

    // Done
    status_page
}
