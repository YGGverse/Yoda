use gtk::{TextTag, WrapMode};

pub fn new() -> TextTag {
    TextTag::builder()
        .left_margin(28)
        .wrap_mode(WrapMode::Word)
        .build()
}
