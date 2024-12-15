use super::MARGIN;
use gtk::{gdk::Cursor, Button};

pub fn new() -> Button {
    Button::builder()
        .cursor(&Cursor::from_name("default", None).unwrap())
        .icon_name("window-close-symbolic")
        .margin_bottom(MARGIN)
        .margin_end(MARGIN)
        .margin_start(MARGIN)
        .margin_top(MARGIN)
        .tooltip_text("Close find bar")
        .build()
}
