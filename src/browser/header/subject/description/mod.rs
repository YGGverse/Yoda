use gtk::prelude::WidgetExt;
use gtk::{pango::EllipsizeMode, Label};

pub struct Description {
    widget: Label,
}

impl Description {
    // Construct
    pub fn new() -> Description {
        let widget = Label::builder()
            .css_classes(["subtitle"])
            .single_line_mode(true)
            .ellipsize(EllipsizeMode::End)
            .visible(false)
            .build();

        Self { widget }
    }

    // Actions
    pub fn update(&self, text: &str) {
        self.widget.set_text(text);
        self.widget.set_visible(!text.is_empty());
    }

    // Getters
    pub fn widget(&self) -> &Label {
        &self.widget
    }
}
