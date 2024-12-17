use gtk::{TextTag, WrapMode};

pub fn new() -> TextTag {
    TextTag::builder()
        .scale(1.6)
        .sentence(true)
        .weight(500)
        .wrap_mode(WrapMode::Word)
        .build()
}
