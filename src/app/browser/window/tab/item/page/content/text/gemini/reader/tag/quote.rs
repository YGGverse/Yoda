use gtk::{pango::Style, TextTag, WrapMode};

pub struct Quote {
    pub text_tag: TextTag,
}

impl Quote {
    // Construct
    pub fn new() -> Self {
        Self {
            text_tag: TextTag::builder()
                .style(Style::Italic)
                .wrap_mode(WrapMode::Word)
                .build(),
        }
    }
}
