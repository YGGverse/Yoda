use adw::Clamp;
use gtk::{prelude::WidgetExt, Box};
use std::sync::Arc;

pub struct Widget {
    gobject: Clamp,
}

impl Widget {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        let gobject = Clamp::builder()
            .css_classes(["app-notification"])
            .maximum_size(800)
            .visible(false)
            .build();

        Arc::new(Self { gobject })
    }

    // Actions
    pub fn update(&self, child: Option<&Box>) {
        if child.is_some() {
            self.gobject.set_child(child);
            self.gobject.set_visible(true);
        } else {
            self.gobject.set_visible(false)
        }
    }

    // Getters
    pub fn gobject(&self) -> &Clamp {
        &self.gobject
    }
}
