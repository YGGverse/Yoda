use gtk::{prelude::WidgetExt, Align, Label};
use std::sync::Arc;

pub struct Widget {
    gobject: Label,
}

impl Widget {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        let gobject = Label::builder()
            .css_classes(["heading"])
            .halign(Align::Start)
            .margin_end(8)
            .margin_start(8)
            .visible(false)
            .build();

        Arc::new(Self { gobject })
    }

    // Actions
    pub fn update(&self, text: Option<&str>) {
        match text {
            Some(value) => {
                self.gobject.set_label(value);
                self.gobject.set_visible(!value.is_empty());
            }
            None => {
                self.gobject.set_visible(false);
            }
        }
    }

    // Getters
    pub fn gobject(&self) -> &Label {
        &self.gobject
    }
}
