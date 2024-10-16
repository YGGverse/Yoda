use gtk::{prelude::BoxExt, Align, Box, Button, Label, Orientation};
use std::sync::Arc;

const SPACING: i32 = 8;

pub struct Widget {
    gobject: Box,
}

impl Widget {
    // Construct
    pub fn new_arc(limit: &Label, send: &Button) -> Arc<Self> {
        let gobject = Box::builder()
            .halign(Align::End)
            .orientation(Orientation::Horizontal)
            .spacing(SPACING)
            .build();

        gobject.append(limit);
        gobject.append(send);

        Arc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
