use gtk::{gdk::RGBA, TextTag};

pub fn new() -> TextTag {
    TextTag::builder()
        .background_rgba(&RGBA::new(0.0, 0.0, 0.0, 1.0))
        .foreground_rgba(&RGBA::new(1.0, 1.0, 1.0, 1.0))
        .build()
}
