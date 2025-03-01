use gtk::{TextTag, WrapMode};

/// Default [TextTag](https://docs.gtk.org/gtk4/class.TextTag.html) preset
/// for ANSI buffer
pub struct Tag {
    pub text_tag: TextTag,
}

impl Default for Tag {
    fn default() -> Self {
        Self::new()
    }
}

impl Tag {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            text_tag: TextTag::builder()
                .family("monospace") // @TODO
                .left_margin(28)
                .scale(0.8)
                .wrap_mode(WrapMode::None)
                .build(),
        }
    }
}
