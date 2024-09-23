use gtk::Image;

pub struct Pin {
    widget: Image,
}

impl Pin {
    // Construct
    pub fn new(visible: bool) -> Pin {
        let widget = Image::builder()
            .icon_name("view-pin-symbolic")
            .visible(visible)
            .build();

        Self { widget }
    }

    // Getters
    pub fn widget(&self) -> &Image {
        &self.widget
    }
}
