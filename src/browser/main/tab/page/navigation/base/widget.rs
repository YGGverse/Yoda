pub struct Base {
    gtk: gtk::Button,
}

impl Base {
    // Construct
    pub fn new() -> Base {
        Self {
            gtk: gtk::Button::builder()
                .icon_name("go-home-symbolic")
                .tooltip_text("Base")
                .sensitive(false)
                .build(),
        }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Button {
        &self.gtk
    }
}
