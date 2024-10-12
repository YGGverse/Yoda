use gtk::{prelude::BoxExt, Box, Button, Orientation};
use std::sync::Arc;

pub struct Widget {
    gobject: Box,
}

impl Widget {
    // Construct
    pub fn new_arc(back: &Button, forward: &Button) -> Arc<Self> {
        // Init widget
        let gobject = Box::builder()
            .orientation(Orientation::Horizontal)
            .css_classes([
                "linked", // merge childs
            ])
            .build();

        // Compose childs
        gobject.append(back);
        gobject.append(forward);

        // Return activated struct
        Arc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
