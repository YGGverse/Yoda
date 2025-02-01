use gtk::{TextTag, WrapMode};

pub fn new() -> TextTag {
    TextTag::builder()
        .foreground("#c88800") // @TODO optional
        .scale(1.2)
        .sentence(true)
        .weight(400)
        .wrap_mode(WrapMode::Word)
        .build()
}
