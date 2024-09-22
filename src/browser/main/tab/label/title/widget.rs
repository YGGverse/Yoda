pub struct Title {
    gtk: gtk::Label,
}

impl Title {
    // Construct
    pub fn new() -> Title {
        let gtk = gtk::Label::builder()
            .label("New page")
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .width_chars(16)
            .single_line_mode(true)
            .build();

        Self { gtk }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Label {
        &self.gtk
    }
}
