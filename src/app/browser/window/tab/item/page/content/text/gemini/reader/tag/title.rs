use gtk::{TextTag, WrapMode};

pub struct Title {
    tag: TextTag,
}

impl Title {
    // Construct
    pub fn new() -> Self {
        Self {
            tag: TextTag::builder()
                .pixels_above_lines(4)
                .pixels_below_lines(8)
                .weight(500)
                .wrap_mode(WrapMode::None)
                .build(),
        }
    }

    // Getters
    pub fn gobject(&self) -> &TextTag {
        &self.tag
    }
}
