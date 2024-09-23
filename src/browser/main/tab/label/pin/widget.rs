pub struct Pin {
    image: gtk::Image,
}

impl Pin {
    // Construct
    pub fn new() -> Pin {
        let image = gtk::Image::builder()
            .icon_name("view-pin-symbolic")
            .visible(false) //@TODO
            .build();

        Self { image }
    }

    // Getters
    pub fn image(&self) -> &gtk::Image {
        &self.image
    }
}
