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
        let mut name = Vec::new();

        if let Some(value) = text {
            if !value.is_empty() {
                name.push(value);
            }
        }

        name.push(GString::from(DEFAULT_TEXT));

        self.widget.set_text(&name.join(" - "));
    }

    // Getters
    pub fn widget(&self) -> &Label {
        &self.widget
    }
}
