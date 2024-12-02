use gtk::{TextTag, WrapMode};

pub struct Code {
    pub text_tag: TextTag,
}

impl Code {
    // Construct
    pub fn new() -> Self {
        Self {
            text_tag: TextTag::builder()
                .family("monospace") // @TODO
                .left_margin(28)
                .scale(0.8)
                .wrap_mode(WrapMode::None)
                .build(),
        }
    }
}
