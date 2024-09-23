mod content;
mod navigation;

use gtk::prelude::BoxExt;
use gtk::{Box, Orientation};

pub struct Page {
    widget: Box,
}

impl Page {
    pub fn new() -> Page {
        // Init components
        let navigation = navigation::Navigation::new();
        let content = content::Content::new();

        // Init widget
        let widget = Box::builder().orientation(Orientation::Vertical).build();

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
