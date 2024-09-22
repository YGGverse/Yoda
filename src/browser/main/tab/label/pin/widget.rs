pub struct Pin {
    gtk: gtk::Image,
}

impl Pin {
    // Construct
    pub fn new() -> Pin {
        let gtk = gtk::Image::builder()
            .icon_name("view-pin-symbolic")
            .visible(false) //@TODO
            .build();

        Self { gtk }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Image {
        &self.gtk
    }
}
