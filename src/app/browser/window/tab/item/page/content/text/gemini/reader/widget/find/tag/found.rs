use gtk::{gdk::RGBA, TextTag};

pub fn new() -> TextTag {
    TextTag::builder()
        .background_rgba(&RGBA::new(0.502, 0.502, 0.502, 0.5)) // @TODO
        .build()
}
