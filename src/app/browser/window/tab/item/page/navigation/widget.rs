use gtk::{prelude::BoxExt, Box, Button, Entry, Orientation};
use std::sync::Arc;

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
            .spacing(8)
            .margin_start(6)
            .margin_end(6)
            .margin_bottom(6)
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
