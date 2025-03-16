use gtk::{Align, Button, prelude::WidgetExt};

pub struct Back {
    pub button: Button,
}

impl Default for Back {
    fn default() -> Self {
        Self::new()
    }
}

impl Back {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            button: Button::builder()
                .icon_name("go-up-symbolic")
                .sensitive(false)
                .tooltip_text("Back")
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
