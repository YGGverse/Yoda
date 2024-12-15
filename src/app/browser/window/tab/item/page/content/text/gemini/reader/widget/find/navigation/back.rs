use super::MARGIN;
use gtk::{gdk::Cursor, Button};

pub fn new() -> Button {
    Button::builder()
        .cursor(&Cursor::from_name("default", None).unwrap())
        .icon_name("go-previous-symbolic")
        .margin_bottom(MARGIN)
        .margin_top(MARGIN)
        .sensitive(false)
        .tooltip_text("Back")
        .build()
}
