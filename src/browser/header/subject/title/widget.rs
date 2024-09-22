const DEFAULT_TEXT: &str = "Yoda";

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
            .label(DEFAULT_TEXT)
            .build();

        Self { gtk }
    }

    // Actions
    pub fn update(&self, text: &str) {
        if text.is_empty() {
            self.gtk.set_text(DEFAULT_TEXT);
        } else {
            self.gtk.set_text(&format!("{} - {}", text, DEFAULT_TEXT));
        }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Label {
        &self.gtk
    }
}
