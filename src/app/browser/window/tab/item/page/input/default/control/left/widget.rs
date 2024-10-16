use gtk::{prelude::WidgetExt, Label};
use std::sync::Arc;

pub struct Widget {
    gobject: Label,
}

impl Widget {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        let gobject = Label::builder().build();

        Arc::new(Self { gobject })
    }

    // Actions
    pub fn update(&self, left: Option<usize>) {
        match left {
            Some(value) => {
                // Update color on limit reached
                self.gobject
                    .set_css_classes(&[if value > 0 { "success" } else { "error" }]); // @TODO add warning step?

                // Update text
                self.gobject.set_label(&value.to_string());

                // Toggle visibility if limit provided
                self.gobject.set_visible(true);
            }
            None => self.gobject.set_visible(false),
        }
    }

    // Getters
    pub fn gobject(&self) -> &Label {
        &self.gobject
    }
}
