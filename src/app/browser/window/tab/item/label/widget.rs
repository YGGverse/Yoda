use std::sync::Arc;

use gtk::{
    glib::GString, prelude::BoxExt, prelude::WidgetExt, Align, Box, Image, Label, Orientation,
};

pub struct Widget {
    gobject: Box,
}

impl Widget {
    // Construct
    pub fn new(name: GString, pin: &Image, title: &Label) -> Arc<Self> {
        let gobject = Box::builder()
            .orientation(Orientation::Horizontal)
            .halign(Align::Center)
            .name(name)
            .tooltip_text(title.text())
            .build();

        gobject.append(pin);
        gobject.append(title);

        Arc::new(Self { gobject })
    }

    // Action
    pub fn update(&self, title: Option<&GString>) {
        match title {
            Some(tooltip_text) => self.gobject.set_tooltip_text(Some(tooltip_text)),
            None => self.gobject.set_tooltip_text(None),
        }
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
