use adw::Banner;

pub struct Notice {
    pub banner: Banner,
}

impl Notice {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        let banner = Banner::builder().button_label("Ok").build();
        banner.connect_button_clicked(|this| this.set_revealed(false));
        Self { banner }
    }

    // Actions

    pub fn show(&self, title: &str) {
        self.banner.set_title(title);
        self.banner.set_revealed(true);
    }

    pub fn unset(&self) {
        self.banner.set_revealed(false);
    }
}

impl Default for Notice {
    fn default() -> Self {
        Self::new()
    }
}
