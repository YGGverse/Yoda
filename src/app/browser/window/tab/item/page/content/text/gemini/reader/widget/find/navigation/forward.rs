use super::MARGIN;
use gtk::Button;

pub fn new() -> Button {
    Button::builder()
        .icon_name("go-next-symbolic")
        .margin_bottom(MARGIN)
        .margin_top(MARGIN)
        .sensitive(false)
        .tooltip_text("Forward")
        .build()
}
