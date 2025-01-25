use gtk::Button;

pub trait Back {
    fn back(action_name: &str) -> Self;
}

impl Back for Button {
    fn back(action_name: &str) -> Self {
        Button::builder()
            .action_name(action_name)
            .icon_name("go-previous-symbolic")
            .tooltip_text("Back")
            .build()
    }
}
