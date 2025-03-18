use gtk::{Button, prelude::WidgetExt};

pub trait Send {
    fn send() -> Self;
    fn update(&self, is_sensitive: bool);
}

impl Send for Button {
    // Constructors

    /// Build new `Self`
    fn send() -> Self {
        Button::builder()
            .css_classes(["accent"]) // | `suggested-action`
            .label("Send")
            .sensitive(false)
            .build()
    }

    // Actions
    fn update(&self, is_sensitive: bool) {
        self.set_sensitive(is_sensitive);
    }
}
