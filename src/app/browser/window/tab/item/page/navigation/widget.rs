use gtk::{prelude::BoxExt, Box, Button, Entry, Orientation};
use std::sync::Arc;

const MARGIN: i32 = 6;
const SPACING: i32 = 6;

pub struct Widget {
    gobject: Box,
}

impl Widget {
    // Construct
    pub fn new_arc(
        base: &Button,
        history: &Box,
        reload: &Button,
        request: &Entry,
        bookmark: &Button,
    ) -> Arc<Self> {
        let gobject = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(SPACING)
            .margin_start(MARGIN)
            .margin_end(MARGIN)
            .margin_bottom(MARGIN)
            .build();

        gobject.append(base);
        gobject.append(history);
        gobject.append(reload);
        gobject.append(request);
        gobject.append(bookmark);

        Arc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
