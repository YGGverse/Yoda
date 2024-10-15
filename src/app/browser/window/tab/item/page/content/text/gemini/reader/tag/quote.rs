use gtk::{pango::Style, TextTag, WrapMode};

pub struct Quote {
    tag: TextTag,
}

impl Quote {
    // Construct
    pub fn new() -> Self {
        Self {
            tag: TextTag::builder()
                .style(Style::Italic)
                .wrap_mode(WrapMode::Word)
                .build(),
        }
    }

    // Getters
    pub fn gobject(&self) -> &TextTag {
        &self.tag
    }
}
