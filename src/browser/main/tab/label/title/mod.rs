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
    pub fn update(&self, title: &GString) {
        self.widget.set_text(title);
    }

    // Getters
    pub fn widget(&self) -> &Label {
        &self.widget
    }
}
