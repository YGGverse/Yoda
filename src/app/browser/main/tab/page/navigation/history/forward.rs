use gtk::Button;

pub fn new() -> Button {
    Button::builder()
        .icon_name("go-next-symbolic")
        .tooltip_text("Forward")
        .sensitive(false)
        .build()
}
