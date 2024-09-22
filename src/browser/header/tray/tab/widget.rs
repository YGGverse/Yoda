pub struct Tab {
    gtk: gtk::Button,
}

impl Tab {
    // Construct
    pub fn new() -> Tab {
        Self {
            gtk: gtk::Button::builder()
                .action_name("win.tab_append")
                .icon_name("tab-new-symbolic")
                .tooltip_text("New tab")
                .build(),
        }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Button {
        &self.gtk
    }
}
