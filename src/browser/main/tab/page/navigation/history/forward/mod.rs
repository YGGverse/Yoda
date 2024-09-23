use gtk::Button;

pub struct Forward {
    widget: Button,
}

impl Forward {
    // Construct
    pub fn new() -> Forward {
        Self {
            widget: Button::builder()
                .icon_name("go-next-symbolic")
                .tooltip_text("Forward")
                .sensitive(false)
                .build(),
        }
    }

    // Getters
    pub fn widget(&self) -> &Button {
        &self.widget
    }
}
