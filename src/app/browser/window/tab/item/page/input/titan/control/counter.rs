use gtk::{prelude::WidgetExt, Label};

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
            label: Label::builder().css_classes(["dim-label"]).build(), // @TODO use `dimmed` in Adw 1.6,
        }
    }

    // Actions
    pub fn update(&self, bytes_total: Option<usize>) {
        match bytes_total {
            Some(value) => {
                self.label.set_label(&value.to_string());
                self.label.set_visible(value > 0);
            }
            None => self.label.set_visible(false),
        }
    }
}
