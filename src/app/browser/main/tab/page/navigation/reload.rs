use gtk::Button;

pub fn new() -> Button {
    return Button::builder()
        .icon_name("view-refresh-symbolic")
        .tooltip_text("Reload")
        .sensitive(false)
        .build();
}
