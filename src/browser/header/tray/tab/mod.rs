use gtk::Button;

pub fn new() -> Button {
    let tab = Button::builder()
        .icon_name("tab-new-symbolic")
        .tooltip_text("New tab")
        .build();

    return tab;
}
