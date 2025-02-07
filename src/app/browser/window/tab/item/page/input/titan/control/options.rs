use gtk::Button;

pub trait Options {
    fn options() -> Self;
}

impl Options for Button {
    fn options() -> Self {
        Button::builder()
            .icon_name("emblem-system-symbolic")
            .tooltip_text("Options")
            .build()
    }
}
