use gtk::{prelude::BoxExt, Align, Box, Button, Label, Orientation};

const MARGIN: i32 = 8;
const SPACING: i32 = 8;

pub struct Widget {
    pub g_box: Box,
}

impl Widget {
    // Construct
    pub fn new(limit: &Label, send: &Button) -> Self {
        // Init main widget
        let g_box = Box::builder()
            .halign(Align::End)
            .margin_bottom(MARGIN)
            .orientation(Orientation::Horizontal)
            .spacing(SPACING)
            .build();

        g_box.append(limit);
        g_box.append(send);

        // Return new `Self`
        Self { g_box }
    }
}
