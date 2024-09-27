use gtk::{glib::GString, pango::EllipsizeMode, Label};

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
    pub fn update(&self, text: Option<GString>) {
        match text {
            Some(value) => self
                .widget
                .set_text(&format!("{} - {}", value, DEFAULT_TEXT)),
            None => self.widget.set_text(DEFAULT_TEXT),
        };
    }

    // Getters
    pub fn widget(&self) -> &Label {
        &self.widget
    }
}
