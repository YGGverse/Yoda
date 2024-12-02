use gtk::{TextTag, WrapMode};

pub struct H3 {
    pub text_tag: TextTag,
}

impl H3 {
    // Construct
    pub fn new() -> Self {
        Self {
            text_tag: TextTag::builder()
                .scale(1.2)
                .sentence(true)
                .weight(400)
                .wrap_mode(WrapMode::Word)
                .build(),
        }
    }
}
