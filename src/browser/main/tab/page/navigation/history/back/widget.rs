pub struct Back {
    gtk: gtk::Button,
}

impl Back {
    // Construct
    pub fn new() -> Back {
        Self {
            gtk: gtk::Button::builder()
                .icon_name("go-previous-symbolic")
                .tooltip_text("Back")
                .sensitive(false)
                .build(),
        }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Button {
        &self.gtk
    }
}
