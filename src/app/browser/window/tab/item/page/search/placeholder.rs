use gtk::{Align, Label, prelude::WidgetExt};

const MARGIN: i32 = 6;

pub struct Placeholder {
    pub label: Label,
}

impl Default for Placeholder {
    fn default() -> Self {
        Self::new()
    }
}

impl Placeholder {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            label: Label::builder()
                .css_classes(["error"])
                .halign(Align::Start)
                .label("Search action requires activation!")
                .margin_start(MARGIN)
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
