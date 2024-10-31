use adw::{Spinner, StatusPage};
use gtk::{
    glib::{timeout_add_local, ControlFlow},
    prelude::WidgetExt,
};
use std::time::Duration;

/// 16-64 (px)
const SPINNER_SIZE: i32 = 64;

pub struct Loading {
    gobject: StatusPage,
}

impl Loading {
    pub fn new(
        title: Option<&str>,
        description: Option<&str>,
        show_with_delay: Option<Duration>,
    ) -> Self {
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

        Self { gobject }
    }

    pub fn gobject(&self) -> &StatusPage {
        &self.gobject
    }
}
