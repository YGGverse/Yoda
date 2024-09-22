pub struct Title {
    gtk: gtk::Label,
}

impl Title {
    // Construct
    pub fn new() -> Title {
        let gtk = gtk::Label::builder()
            .css_classes(["title"])
            .single_line_mode(true)
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .build();

        Self { gtk }
    }

    // Actions
    pub fn update(&self, text: &str) {
        let default_text = "Yoda"; // @TODO

        if text.is_empty() {
            self.gtk.set_text(default_text);
        } else {
            self.gtk.set_text(&format!("{} - {}", text, default_text));
        }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Label {
        &self.gtk
    }
}
