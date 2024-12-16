use gtk::{gdk::Cursor, Button};

const MARGIN: i32 = 6;

pub fn new() -> Button {
    Button::builder()
        .cursor(&Cursor::from_name("default", None).unwrap())
        .icon_name("window-close-symbolic")
        .margin_end(MARGIN)
        .margin_start(MARGIN)
        .tooltip_text("Close find bar")
        .build()
}
