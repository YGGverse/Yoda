use gtk::{TextTag, WrapMode};

pub fn new() -> TextTag {
    TextTag::builder()
        .scale(1.4)
        .sentence(true)
        .weight(400)
        .wrap_mode(WrapMode::Word)
        .build()
}
