use gtk::Button;

pub trait Forward {
    fn forward(action_name: &str) -> Self;
}

impl Forward for Button {
    fn forward(action_name: &str) -> Self {
        Button::builder()
            .action_name(action_name)
            .icon_name("go-next-symbolic")
            .tooltip_text("Forward")
            .build()
    }
}
