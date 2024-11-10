use adw::PasswordEntryRow;
use gtk::{prelude::BoxExt, Box, Orientation};

const MARGIN: i32 = 6;
const SPACING: i32 = 8;

pub struct Widget {
    gobject: Box,
}

impl Widget {
    // Construct
    pub fn new(response: &PasswordEntryRow) -> Self {
        let gobject = Box::builder()
            .margin_bottom(MARGIN)
            .margin_end(MARGIN)
            .margin_start(MARGIN)
            .margin_top(MARGIN)
            .spacing(SPACING)
            .orientation(Orientation::Vertical)
            .build();

        gobject.append(response);

        Self { gobject }
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
