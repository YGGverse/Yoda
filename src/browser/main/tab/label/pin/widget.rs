pub struct Pin {
    image: gtk::Image,
}

impl Pin {
    // Construct
    pub fn new(is_pinned: bool) -> Pin {
        let image = gtk::Image::builder()
            .icon_name("view-pin-symbolic")
            .visible(is_pinned)
            .build();

        Self { image }
    }

    // Getters
    pub fn image(&self) -> &gtk::Image {
        &self.image
    }
}
