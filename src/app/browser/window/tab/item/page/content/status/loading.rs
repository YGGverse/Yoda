use adw::{Spinner, StatusPage};
use gtk::{
    glib::{timeout_add_local, ControlFlow},
    prelude::WidgetExt,
};
use std::time::Duration;

const SPINNER_SIZE: i32 = 64; // 16-64
const DEFAULT_TITLE: &str = "Loading..";

/// Create new default `GObject` preset for loading
/// [StatusPage](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.StatusPage.html)
pub fn new_gobject(show_with_delay: Option<Duration>) -> StatusPage {
    let gobject = StatusPage::builder()
        .child(
            &Spinner::builder()
                .width_request(SPINNER_SIZE)
                .height_request(SPINNER_SIZE)
                .build(),
        )
        .title(DEFAULT_TITLE)
        .build();

    if let Some(duration) = show_with_delay {
        gobject.set_visible(false);
        timeout_add_local(duration, {
            let this = gobject.clone();
            move || {
                this.set_visible(true);
                ControlFlow::Break
            }
        });
    }

    gobject
}
