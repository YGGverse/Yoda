use gtk::{glib::GString, Box, Orientation};

pub struct Content {
    widget: Box,
}

impl Content {
    // Construct
    pub fn new() -> Content {
        Self {
            widget: Box::builder().orientation(Orientation::Vertical).build(),
        }
    }

    // Actions
    pub fn reload(&self, request_text: GString) {
        // @TODO
    }

    // Getters
    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
