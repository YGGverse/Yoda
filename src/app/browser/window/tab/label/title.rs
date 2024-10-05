use gtk::{glib::GString, pango::EllipsizeMode, Label};

const DEFAULT_LABEL_TEXT: &str = "New page";

pub struct Title {
    widget: Label,
}

impl Title {
    // Construct
    pub fn new() -> Self {
        Self {
            widget: Label::builder()
                .label(DEFAULT_LABEL_TEXT)
                .ellipsize(EllipsizeMode::End)
                .width_chars(16)
                .single_line_mode(true)
                .build(),
        }
    }

    // Actions
    pub fn update(&self, title: Option<&GString>) {
        match title {
            Some(title) => self.widget.set_text(title),
            None => self.widget.set_text(DEFAULT_LABEL_TEXT),
        }
    }

    // Getters
    pub fn widget(&self) -> &Label {
        &self.widget
    }
}
