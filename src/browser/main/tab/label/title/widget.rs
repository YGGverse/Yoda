pub struct Title {
    label: gtk::Label,
}

impl Title {
    // Construct
    pub fn new() -> Title {
        let label = gtk::Label::builder()
            .label("New page")
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .width_chars(16)
            .single_line_mode(true)
            .build();

        Self { label }
    }

    // Getters
    pub fn label(&self) -> &gtk::Label {
        &self.label
    }
}
