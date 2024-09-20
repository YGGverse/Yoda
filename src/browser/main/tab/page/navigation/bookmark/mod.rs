use gtk::Button;

pub fn new() -> Button {
    Button::builder()
        .icon_name("starred-symbolic")
        .tooltip_text("Toggle bookmark")
        .sensitive(false)
        .build()
}
