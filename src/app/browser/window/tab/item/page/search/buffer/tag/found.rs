use gtk::{gdk::RGBA, TextTag};

pub fn new() -> TextTag {
    TextTag::builder()
        .background_rgba(&RGBA::new(0.5, 0.5, 0.5, 0.5)) // @TODO use accent colors after adw 1.6 update
        .build()
}
