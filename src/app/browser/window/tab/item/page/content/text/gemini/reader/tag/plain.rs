use gtk::{TextTag, WrapMode};

pub struct Plain {
    pub text_tag: TextTag,
}

impl Plain {
    // Construct
    pub fn new() -> Self {
        Self {
            text_tag: TextTag::builder().wrap_mode(WrapMode::Word).build(),
        }
    }
}
