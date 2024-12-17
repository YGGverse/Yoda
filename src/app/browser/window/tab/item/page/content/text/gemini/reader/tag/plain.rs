use gtk::{TextTag, WrapMode};

pub fn new() -> TextTag {
    TextTag::builder().wrap_mode(WrapMode::Word).build()
}
