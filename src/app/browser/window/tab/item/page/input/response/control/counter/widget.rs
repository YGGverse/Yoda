use gtk::{prelude::WidgetExt, Label};

pub struct Widget {
    pub label: Label,
}

impl Default for Widget {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget {
    // Construct
    pub fn new() -> Self {
        Self {
            label: Label::builder().build(),
        }
    }

    // Actions
    pub fn update(&self, chars_left: Option<i32>) {
        match chars_left {
            Some(value) => {
                // Update color on chars left reached
                self.label
                    .set_css_classes(&[if value > 0 { "success" } else { "error" }]); // @TODO add warning step?

                // Update text
                self.label.set_label(&value.to_string());

                // Toggle visibility on chars left provided
                self.label.set_visible(true);
            }
            None => self.label.set_visible(false),
        }
    }
}
