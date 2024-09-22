pub struct Tab {
    gtk: gtk::Button,
}

impl Tab {
    // Construct
    pub fn new() -> Tab {
        let gtk = gtk::Button::builder()
            .icon_name("tab-new-symbolic")
            .tooltip_text("New tab")
            .build();

        Self { gtk }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Button {
        &self.gtk
    }
}
