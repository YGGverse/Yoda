use gtk::{Align, Button, prelude::WidgetExt};

pub struct Forward {
    pub button: Button,
}

impl Default for Forward {
    fn default() -> Self {
        Self::new()
    }
}

impl Forward {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            button: Button::builder()
                .icon_name("go-down-symbolic")
                .sensitive(false)
                .tooltip_text("Forward")
                .valign(Align::Center)
                .vexpand(false)
                .build(),
        }
    }

    // Actions

    pub fn update(&self, is_sensitive: bool) {
        self.button.set_sensitive(is_sensitive);
    }
}
