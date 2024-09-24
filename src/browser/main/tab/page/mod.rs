mod content;
mod navigation;

use gtk::{glib::GString, prelude::BoxExt, Box, Orientation};

pub struct Page {
    widget: Box,
}

impl Page {
    pub fn new(name: GString) -> Page {
        // Init components
        let navigation = navigation::Navigation::new();
        let content = content::Content::new();

        // Init widget
        let widget = Box::builder()
            .orientation(Orientation::Vertical)
            .name(name)
            .build();

        widget.append(navigation.widget());
        widget.append(content.widget());

        // Result
        Self { widget }
    }

    // Getters
    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
