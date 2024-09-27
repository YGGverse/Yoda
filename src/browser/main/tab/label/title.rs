use gtk::{glib::GString, pango::EllipsizeMode, Label};

pub struct Title {
    widget: Label,
}

impl Title {
    // Construct
    pub fn new() -> Self {
        Self {
            widget: Label::builder()
                .label("New page")
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
            None => self.widget.set_text(""), // @TODO None/false option
        }
    }

    // Getters
    pub fn widget(&self) -> &Label {
        &self.widget
    }
}
