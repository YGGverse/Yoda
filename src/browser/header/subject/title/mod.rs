use gtk::{pango::EllipsizeMode, Label};

const DEFAULT_TEXT: &str = "Yoda"; // @TODO

pub struct Title {
    widget: Label,
}

impl Title {
    // Construct
    pub fn new() -> Self {
        let widget = gtk::Label::builder()
            .css_classes(["title"])
            .single_line_mode(true)
            .ellipsize(EllipsizeMode::End)
            .label(DEFAULT_TEXT)
            .build();

        Self { widget }
    }

    // Actions
    pub fn update(&self, text: &str) {
        if text.is_empty() {
            self.widget.set_text(DEFAULT_TEXT);
        } else {
            self.widget
                .set_text(&format!("{} - {}", text, DEFAULT_TEXT));
        }
    }

    // Getters
    pub fn widget(&self) -> &Label {
        &self.widget
    }
}
