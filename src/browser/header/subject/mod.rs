mod description;
mod title;

use description::Description;
use gtk::prelude::BoxExt;
use gtk::{Align, Box, Orientation};
use title::Title;

pub struct Subject {
    widget: Box,
}

impl Subject {
    // Construct
    pub fn new() -> Subject {
        let title = Title::new();
        let description = Description::new();

        let widget = Box::builder()
            .orientation(Orientation::Vertical)
            .valign(Align::Center)
            .build();

        widget.append(title.widget());
        widget.append(description.widget());

        Self { widget }
    }

    // Getters
    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
