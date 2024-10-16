use gtk::{Align, Button};
use std::sync::Arc;

pub struct Widget {
    gobject: Button,
}

impl Widget {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        let gobject = Button::builder()
            .css_classes(["accent"])
            .halign(Align::End)
            .label("Send")
            .build();

        Arc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        &self.gobject
    }
}
