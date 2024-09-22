pub struct Forward {
    gtk: gtk::Button,
}

impl Forward {
    // Construct
    pub fn new() -> Forward {
        Self {
            gtk: gtk::Button::builder()
                .icon_name("go-next-symbolic")
                .tooltip_text("Forward")
                .sensitive(false)
                .build(),
        }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Button {
        &self.gtk
    }
}
