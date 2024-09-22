pub struct Bookmark {
    gtk: gtk::Button,
}

impl Bookmark {
    // Construct
    pub fn new() -> Bookmark {
        Self {
            gtk: gtk::Button::builder()
                .icon_name("starred-symbolic")
                .tooltip_text("Toggle bookmark")
                .sensitive(false)
                .build(),
        }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Button {
        &self.gtk
    }
}
