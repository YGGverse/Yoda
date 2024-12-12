use gtk::{prelude::WidgetExt, Label};

// Defaults

const CSS_CLASSES_DEFAULT: &[&str; 0] = &[];
const CSS_CLASSES_ERROR: &[&str; 1] = &["error"];
const CSS_CLASSES_SUCCESS: &[&str; 1] = &["success"];
const CSS_CLASSES_WARNING: &[&str; 1] = &["warning"];
const MARGIN: i32 = 16;

/// Indicate current download state as the text
/// [Label](https://docs.gtk.org/gtk4/class.Label.html)
pub struct Status {
    pub label: Label,
}

impl Status {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            label: Label::builder().visible(false).margin_top(MARGIN).build(),
        }
    }

    // Actions

    pub fn set_default(&self, label: &str) {
        self.label.set_css_classes(CSS_CLASSES_DEFAULT);
        self.label.set_label(label);
        self.label.set_visible(true)
    }

    pub fn set_error(&self, label: &str) {
        self.label.set_css_classes(CSS_CLASSES_ERROR);
        self.label.set_label(label);
        self.label.set_visible(true)
    }

    pub fn set_success(&self, label: &str) {
        self.label.set_css_classes(CSS_CLASSES_SUCCESS);
        self.label.set_label(label);
        self.label.set_visible(true)
    }

    pub fn set_warning(&self, label: &str) {
        self.label.set_css_classes(CSS_CLASSES_WARNING);
        self.label.set_label(label);
        self.label.set_visible(true)
    }
}
