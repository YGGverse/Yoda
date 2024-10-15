use gtk::{TextTag, WrapMode};

pub struct H2 {
    tag: TextTag,
}

impl H2 {
    // Construct
    pub fn new() -> Self {
        Self {
            tag: TextTag::builder()
                .scale(1.4)
                .sentence(true)
                .weight(400)
                .wrap_mode(WrapMode::Word)
                .build(),
        }
    }

    // Getters
    pub fn gobject(&self) -> &TextTag {
        &self.tag
    }
}
