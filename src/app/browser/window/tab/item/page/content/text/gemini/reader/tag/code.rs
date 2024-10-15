use gtk::{TextTag, WrapMode};

pub struct Code {
    tag: TextTag,
}

impl Code {
    // Construct
    pub fn new() -> Self {
        Self {
            tag: TextTag::builder()
                .family("monospace") // @TODO
                .scale(0.8)
                .wrap_mode(WrapMode::None)
                .build(),
        }
    }

    // Getters
    pub fn gobject(&self) -> &TextTag {
        &self.tag
    }
}
