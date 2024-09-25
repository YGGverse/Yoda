mod description;
mod title;

use description::Description;
use title::Title;

use gtk::{glib::GString, prelude::BoxExt, Align, Box, Orientation};

pub struct Subject {
    widget: Box,
    title: Title,
    description: Description,
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

        Self {
            widget,
            title,
            description,
        }
    }

    // Actions
    pub fn update(&self, title: GString, description: GString) {
        self.title.update(&title);
        self.description.update(&description);
    }

    // Getters
    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
