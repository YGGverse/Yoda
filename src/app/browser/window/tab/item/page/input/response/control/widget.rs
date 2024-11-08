use gtk::{prelude::BoxExt, Align, Box, Button, Label, Orientation};
use std::rc::Rc;

const SPACING: i32 = 8;

pub struct Widget {
    gobject: Box,
}

impl Widget {
    // Construct
    pub fn new_rc(limit: &Label, send: &Button) -> Rc<Self> {
        // Init gobject
        let gobject = Box::builder()
            .halign(Align::End)
            .orientation(Orientation::Horizontal)
            .spacing(SPACING)
            .build();

        gobject.append(limit);
        gobject.append(send);

        // Return new struct
        Rc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
