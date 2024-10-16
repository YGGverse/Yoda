use adw::Clamp;
use gtk::{prelude::WidgetExt, Box};
use std::sync::Arc;

pub struct Widget {
    gobject: Clamp,
}

impl Widget {
    // Construct
    pub fn new_arc(child: &Box) -> Arc<Self> {
        let gobject = Clamp::builder()
            .child(child)
            .css_classes(["app-notification"])
            .maximum_size(800)
            .visible(false)
            .build();

        Arc::new(Self { gobject })
    }

    // Actions
    pub fn show(&self, visible: bool) {
        self.gobject.set_visible(visible);
    }

    // Getters
    pub fn gobject(&self) -> &Clamp {
        &self.gobject
    }
}
