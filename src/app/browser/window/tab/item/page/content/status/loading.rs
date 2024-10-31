use adw::{Spinner, StatusPage};
use gtk::{
    glib::{timeout_add_local, ControlFlow},
    prelude::WidgetExt,
};
use std::time::Duration;

/// 16-64 (px)
const SPINNER_SIZE: i32 = 64;

/// Create new `GObject` preset for loading [StatusPage](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.StatusPage.html)
pub fn new_gobject_from(
    title: Option<&str>,
    description: Option<&str>,
    show_with_delay: Option<Duration>,
) -> StatusPage {
    let gobject = StatusPage::builder()
        .child(
            &Spinner::builder()
                .width_request(SPINNER_SIZE)
                .height_request(SPINNER_SIZE)
                .build(),
        )
        .build();

    if let Some(value) = title {
        gobject.set_title(value);
    }

    gobject.set_description(description);

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
