use gtk::{gdk::RGBA, TextTag, WrapMode};

pub struct Found {
    pub text_tag: TextTag,
}

impl Found {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            text_tag: TextTag::builder()
                .background_rgba(&RGBA::new(0.502, 0.502, 0.502, 0.5)) // @TODO
                .wrap_mode(WrapMode::Word)
                .build(),
        }
    }
}
