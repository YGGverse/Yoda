use adw::Banner;

pub trait Notice {
    fn notice() -> Self;
    fn show(&self, title: &str);
}

impl Notice for Banner {
    // Constructors

    /// Create new `Self`
    fn notice() -> Self {
        let banner = Banner::builder().button_label("Ok").revealed(false).build();
        banner.connect_button_clicked(|this| this.set_revealed(false));
        banner
    }

    // Actions

    fn show(&self, title: &str) {
        self.set_title(title);
        self.set_revealed(true);
    }
}
