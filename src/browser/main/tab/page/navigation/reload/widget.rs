pub struct Reload {
    gtk: gtk::Button,
}

impl Reload {
    // Construct
    pub fn new() -> Reload {
        Self {
            gtk: gtk::Button::builder()
                .icon_name("view-refresh-symbolic")
                .tooltip_text("Reload")
                .sensitive(false)
                .build(),
        }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Button {
        &self.gtk
    }
}
