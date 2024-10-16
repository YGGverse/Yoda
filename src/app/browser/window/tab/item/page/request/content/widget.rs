use gtk::{prelude::BoxExt, Box, Button, Entry, Orientation};
use std::sync::Arc;

const MARGIN: i32 = 6;
const SPACING: i32 = 8;

pub struct Widget {
    gobject: Box,
}

impl Widget {
    // Construct
    pub fn new_arc(response: &Entry, send: &Button) -> Arc<Self> {
        let gobject = Box::builder()
            .margin_bottom(MARGIN)
            .margin_end(MARGIN)
            .margin_start(MARGIN)
            .margin_top(MARGIN)
            .spacing(SPACING)
            .orientation(Orientation::Horizontal)
            .build();

        gobject.append(response);
        gobject.append(send);

        Arc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
