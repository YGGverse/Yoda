mod pin;
mod title;

use pin::Pin;
use title::Title;

use gtk::{
    glib::GString,
    prelude::{BoxExt, WidgetExt},
    Align, Box, Orientation,
};

pub struct Label {
    // Components
    pin: Pin,
    title: Title,

    // GTK
    widget: Box,
}

impl Label {
    // Construct
    pub fn new(name: GString, is_pinned: bool) -> Label {
        // Components
        let pin = Pin::new(is_pinned);
        let title = Title::new();

        // GTK
        let widget = Box::builder()
            .orientation(Orientation::Horizontal)
            .halign(Align::Center)
            .name(name)
            .tooltip_text(title.widget().text())
            .build();

        widget.append(pin.widget());
        widget.append(title.widget());

        // Result
        Self { pin, title, widget }
    }

    // Actions
    pub fn update(&self, title: &GString) {
        self.widget.set_tooltip_text(Some(title));
        self.title.update(title);
    }

    // Setters
    pub fn pin(&self, is_pinned: bool) {
        self.pin.widget().set_visible(is_pinned);
        self.title.widget().set_visible(!is_pinned);
    }

    // Getters
    pub fn is_pinned(&self) -> bool {
        self.pin.widget().is_visible()
    }

    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
