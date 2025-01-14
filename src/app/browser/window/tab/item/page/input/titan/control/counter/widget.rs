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
            label: Label::builder().css_classes(["dim-label"]).build(), // @TODO `.dimmed` Since: Adw 1.7
        }
    }

    // Actions
    pub fn update(&self, bytes: Option<usize>) {
        match bytes {
            Some(value) => {
                self.label.set_label(&crate::tool::format_bytes(value));
                self.label.set_visible(value > 0);
            }
            None => self.label.set_visible(false),
        }
    }
}
