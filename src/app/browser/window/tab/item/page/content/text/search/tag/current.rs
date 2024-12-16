use gtk::{gdk::RGBA, TextTag};

pub fn new() -> TextTag {
    TextTag::builder()
        .background_rgba(&RGBA::new(0.0, 0.3906, 0.8653, 1.0)) // @TODO use accent colors after adw 1.6 update
        .foreground_rgba(&RGBA::new(1.0, 1.0, 1.0, 1.0))
        .build()
}
