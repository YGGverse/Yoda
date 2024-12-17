use gtk::{Align, Button};

const MARGIN: i32 = 6;

pub fn new() -> Button {
    Button::builder()
        .icon_name("window-close-symbolic")
        .margin_end(MARGIN)
        .margin_start(MARGIN)
        .valign(Align::Center)
        .vexpand(false)
        .tooltip_text("Close find bar")
        .build()
}
