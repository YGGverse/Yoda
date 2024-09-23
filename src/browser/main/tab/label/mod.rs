mod pin;
mod title;

use gtk::prelude::{BoxExt, WidgetExt};
use gtk::{Box, Orientation};
use pin::Pin;
use title::Title;

pub struct Label {
    // Components
    pin: Pin,
    title: Title,

    // GTK
    widget: Box,
}

impl Label {
    // Construct
    pub fn new(is_pinned: bool) -> Label {
        // Components
        let pin = Pin::new(is_pinned);
        let title = Title::new();

        // GTK
        let widget = Box::builder().orientation(Orientation::Horizontal).build();

        widget.append(pin.widget());
        widget.append(title.widget());

        // Result
        Self { pin, title, widget }
    }

    // Actions
    pub fn pin(&self) -> bool {
        // Toggle status
        let is_pinned = !self.pin.widget().is_visible();

        // Update pin widget
        self.pin.widget().set_visible(is_pinned);

        // Update label widget
        self.title.widget().set_visible(!is_pinned);

        // Result
        is_pinned
    }

    // Getters
    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
