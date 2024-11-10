use gtk::{prelude::BoxExt, Align, Box, Button, Label, Orientation};

const SPACING: i32 = 8;

pub struct Widget {
    gobject: Box,
}

impl Widget {
    // Construct
    pub fn new(limit: &Label, send: &Button) -> Self {
        // Init gobject
        let gobject = Box::builder()
            .halign(Align::End)
            .orientation(Orientation::Horizontal)
            .spacing(SPACING)
            .build();

        gobject.append(limit);
        gobject.append(send);

        // Return new `Self`
        Self { gobject }
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
