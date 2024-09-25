use gtk::{pango::EllipsizeMode, Label};

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

    // Getters
    pub fn widget(&self) -> &Label {
        &self.widget
    }
}
