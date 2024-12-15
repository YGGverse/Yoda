use gtk::{gdk::Cursor, prelude::WidgetExt, Button};

const MARGIN: i32 = 6;

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
                .icon_name("go-next-symbolic")
                .margin_bottom(MARGIN)
                .margin_top(MARGIN)
                .sensitive(false)
                .tooltip_text("Forward")
                .build(),
        }
    }

    // Actions

    pub fn update(&self, is_sensitive: bool) {
        self.button.set_sensitive(is_sensitive);
    }
}
