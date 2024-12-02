use gtk::{TextTag, WrapMode};

pub struct List {
    pub text_tag: TextTag,
}

impl List {
    // Construct
    pub fn new() -> Self {
        Self {
            text_tag: TextTag::builder()
                .left_margin(28)
                .pixels_above_lines(4)
                .pixels_below_lines(4)
                .wrap_mode(WrapMode::Word)
                .build(),
        }
    }
}
