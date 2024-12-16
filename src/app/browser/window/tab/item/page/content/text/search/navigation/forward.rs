use gtk::{gdk::Cursor, prelude::WidgetExt, Align, Button};

pub struct Forward {
    pub button: Button,
}

impl Forward {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            button: Button::builder()
                .cursor(&Cursor::from_name("default", None).unwrap())
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
