mod description;
mod title;

use description::Description;
use title::Title;

use gtk::{glib::GString, prelude::BoxExt, Align, Box, Orientation};

pub struct Subject {
    gobject: Box,
    title: Title,
    description: Description,
}

impl Subject {
    // Construct
    pub fn new() -> Self {
        let title = Title::new();
        let description = Description::new();

        let gobject = Box::builder()
            .orientation(Orientation::Vertical)
            .valign(Align::Center)
            .build();

        gobject.append(title.gobject());
        gobject.append(description.gobject());

        Self {
            gobject,
            title,
            description,
        }
    }

    // Actions
    pub fn update(&self, title: Option<GString>, description: Option<GString>) {
        self.title.update(title);
        self.description.update(description);
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
