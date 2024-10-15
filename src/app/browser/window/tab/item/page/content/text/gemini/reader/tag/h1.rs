use gtk::{TextTag, WrapMode};

pub struct H1 {
    tag: TextTag,
}

impl H1 {
    // Construct
    pub fn new() -> Self {
        Self {
            tag: TextTag::builder()
                .scale(1.6)
                .sentence(true)
                .weight(500)
                .wrap_mode(WrapMode::Word)
                .build(),
        }
    }

    // Getters
    pub fn gobject(&self) -> &TextTag {
        &self.tag
    }
}
