use adw::Clamp;
use gtk::{prelude::WidgetExt, Box};
use std::rc::Rc;

pub struct Widget {
    gobject: Clamp,
}

impl Widget {
    // Construct
    pub fn new_rc() -> Rc<Self> {
        let gobject = Clamp::builder()
            .css_classes(["app-notification"])
            .maximum_size(800)
            .visible(false)
            .build();

        Rc::new(Self { gobject })
    }

    // Actions
    pub fn update(&self, child: Option<&Box>) {
        if child.is_some() {
            self.gobject.set_visible(true); // widget may be hidden, make it visible to child redraw
            self.gobject.set_child(child);
        } else {
            self.gobject.set_visible(false)
        }
    }

    // Getters
    pub fn gobject(&self) -> &Clamp {
        &self.gobject
    }
}
