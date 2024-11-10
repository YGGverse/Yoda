use gtk::{prelude::WidgetExt, Label};

pub struct Widget {
    gobject: Label,
}

impl Widget {
    // Construct
    pub fn new() -> Self {
        Self {
            gobject: Label::builder().build(),
        }
    }

    // Actions
    pub fn update(&self, chars_left: Option<i32>) {
        match chars_left {
            Some(value) => {
                // Update color on chars left reached
                self.gobject
                    .set_css_classes(&[if value > 0 { "success" } else { "error" }]); // @TODO add warning step?

                // Update text
                self.gobject.set_label(&value.to_string());

                // Toggle visibility on chars left provided
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
