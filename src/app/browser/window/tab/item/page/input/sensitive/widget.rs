use adw::PasswordEntryRow;
use gtk::{prelude::BoxExt, Box, Orientation};

const MARGIN: i32 = 6;
const SPACING: i32 = 8;

pub struct Widget {
    pub g_box: Box,
}

impl Widget {
    // Constructors

    /// Build new `Self`
    pub fn build(response: &PasswordEntryRow) -> Self {
        let g_box = Box::builder()
            .margin_bottom(MARGIN)
            .margin_end(MARGIN)
            .margin_start(MARGIN)
            .margin_top(MARGIN)
            .spacing(SPACING)
            .orientation(Orientation::Vertical)
            .build();

        g_box.append(response);

        Self { g_box }
    }
}
