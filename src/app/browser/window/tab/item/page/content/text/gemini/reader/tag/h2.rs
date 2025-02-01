use gtk::{TextTag, WrapMode};

pub fn new() -> TextTag {
    TextTag::builder()
        .foreground("#d56199") // @TODO optional
        .scale(1.4)
        .sentence(true)
        .weight(400)
        .wrap_mode(WrapMode::Word)
        .build()
}
