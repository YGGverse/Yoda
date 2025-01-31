use gtk::{prelude::WidgetExt, Label};
use plurify::Plurify;

pub struct Counter {
    pub label: Label,
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

impl Counter {
    // Construct
    pub fn new() -> Self {
        Self {
            label: Label::builder().build(),
        }
    }

    // Actions
    pub fn update(&self, is_empty: bool, bytes_left: Option<isize>) {
        match bytes_left {
            Some(value) => {
                // Update color on chars left reached
                self.label.set_css_classes(&[if value.is_positive() {
                    "success"
                } else {
                    "error"
                }]); // @TODO add warning step?

                // Update text
                self.label.set_label(&value.to_string());

                // Toggle visibility on chars left provided
                self.label.set_visible(!is_empty);

                self.label.set_tooltip_text(Some(&format!(
                    "{value} {} left",
                    (value as usize).plurify(&["byte", "bytes", "bytes"])
                )));
            }
            None => self.label.set_visible(false),
        }
    }
}
