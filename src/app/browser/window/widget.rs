use gtk::{prelude::BoxExt, Box, Notebook, Orientation};

pub struct Widget {
    gobject: Box,
}

impl Widget {
    // Construct
    pub fn new(tab: &Notebook) -> Self {
        let gobject = Box::builder().orientation(Orientation::Vertical).build();
        gobject.append(tab);

        Self { gobject }
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
