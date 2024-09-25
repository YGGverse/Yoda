use gtk::{Box, Orientation};

pub struct Content {
    widget: Box,
}

impl Content {
    // Construct
    pub fn new() -> Self {
        Self {
            widget: Box::builder().orientation(Orientation::Vertical).build(),
        }
    }

    // Getters
    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
