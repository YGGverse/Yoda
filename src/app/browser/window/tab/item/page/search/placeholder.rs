use gtk::{prelude::WidgetExt, Label};

pub struct Placeholder {
    pub label: Label,
}

impl Placeholder {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            label: Label::builder()
                .css_classes(["error"])
                .label("Search action requires activation!")
                .build(),
        }
    }

    // Actions

    pub fn show(&self) {
        self.label.set_visible(true)
    }

    pub fn hide(&self) {
        self.label.set_visible(false)
    }
}
