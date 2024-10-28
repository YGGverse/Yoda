use adw::{Spinner, StatusPage};

/// 16-64 (px)
const SPINNER_SIZE: i32 = 64;

pub struct Widget {
    gobject: StatusPage,
}

impl Widget {
    pub fn new(title: Option<&str>, description: Option<&str>) -> Self {
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

        Self { gobject }
    }

    pub fn gobject(&self) -> &StatusPage {
        &self.gobject
    }
}
