use gtk::Button;

pub fn new() -> Button {
    Button::builder()
        .icon_name("go-previous-symbolic")
        .tooltip_text("Back")
        .sensitive(false)
        .build()
}
