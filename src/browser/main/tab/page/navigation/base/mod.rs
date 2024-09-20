use gtk::Button;

pub fn new() -> Button {
    Button::builder()
        .icon_name("go-home-symbolic")
        .tooltip_text("Base")
        .sensitive(false)
        .build()
}
