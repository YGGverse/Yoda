use gtk::{TextTag, WrapMode};

pub struct H2 {
    pub text_tag: TextTag,
}

impl H2 {
    // Construct
    pub fn new() -> Self {
        Self {
            text_tag: TextTag::builder()
                .scale(1.4)
                .sentence(true)
                .weight(400)
                .wrap_mode(WrapMode::Word)
                .build(),
        }
    }
}
