use gtk::{prelude::WidgetExt, Button};
use std::sync::Arc;

pub struct Widget {
    gobject: Button,
}

impl Widget {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        let gobject = Button::builder()
            //.css_classes(["accent"])
            .label("Send")
            .build();

        Arc::new(Self { gobject })
    }

    // Actions
    pub fn update(&self, is_sensitive: bool) {
        self.gobject.set_sensitive(is_sensitive);
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        &self.gobject
    }
}
