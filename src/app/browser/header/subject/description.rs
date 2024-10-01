use gtk::glib::GString;
use gtk::prelude::WidgetExt;
use gtk::{pango::EllipsizeMode, Label};

pub struct Description {
    widget: Label,
}

impl Description {
    // Construct
    pub fn new() -> Self {
        let widget = Label::builder()
            .css_classes(["subtitle"])
            .single_line_mode(true)
            .ellipsize(EllipsizeMode::End)
            .visible(false)
            .build();

        Self { widget }
    }

    // Actions
    pub fn update(&self, text: Option<GString>) {
        match text {
            Some(value) => self.widget.set_text(&value),
            None => self.widget.set_text(""), // @TODO
        };
        self.widget.set_visible(!self.widget.text().is_empty());
    }

    // Getters
    pub fn widget(&self) -> &Label {
        &self.widget
    }
}
