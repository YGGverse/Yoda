use gtk::{TextTag, WrapMode};

pub fn new() -> TextTag {
    TextTag::builder()
        .foreground("#3584e4") // @TODO optional
        .scale(1.6)
        .sentence(true)
        .weight(500)
        .wrap_mode(WrapMode::Word)
        .build()
}
