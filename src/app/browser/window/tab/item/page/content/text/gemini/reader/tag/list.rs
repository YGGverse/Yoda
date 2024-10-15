use gtk::{TextTag, WrapMode};

pub struct List {
    tag: TextTag,
}

impl List {
    // Construct
    pub fn new() -> Self {
        Self {
            tag: TextTag::builder()
                .left_margin(28)
                .pixels_above_lines(4)
                .pixels_below_lines(4)
                .wrap_mode(WrapMode::Word)
                .build(),
        }
    }

    // Getters
    pub fn gobject(&self) -> &TextTag {
        &self.tag
    }
}
