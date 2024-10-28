use adw::StatusPage;

pub struct Widget {
    gobject: StatusPage,
}

impl Widget {
    // Constructors

    /// Create new default widget configuration with options
    pub fn new(title: Option<&str>, description: Option<&str>) -> Self {
        let gobject = StatusPage::new();

        if let Some(value) = title {
            gobject.set_title(value);
        }

        gobject.set_description(description);

        Self { gobject }
    }

    // Getters

    pub fn gobject(&self) -> &StatusPage {
        &self.gobject
    }
}
