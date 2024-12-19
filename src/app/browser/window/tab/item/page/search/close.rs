use gtk::{Align, Button};

const MARGIN: i32 = 6;

pub fn new() -> Button {
    Button::builder()
        .halign(Align::End)
        .hexpand(true)
        .icon_name("window-close-symbolic")
        .margin_end(MARGIN)
        .margin_start(MARGIN)
        .tooltip_text("Close find bar")
        .valign(Align::Center)
        .vexpand(false)
        .build()
}
