use gtk::Button;

pub struct Tab {
    pub widget: Button,
}

impl Tab {
    // Construct
    pub fn new() -> Tab {
        Self {
            widget: Button::builder()
                .action_name("win.tab_append")
                .icon_name("tab-new-symbolic")
                .tooltip_text("New tab")
                .build(),
        }
    }

    // Getters
    pub fn widget(&self) -> &Button {
        &self.widget
    }
}
