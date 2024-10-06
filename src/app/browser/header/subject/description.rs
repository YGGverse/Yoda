use gtk::glib::GString;
use gtk::prelude::WidgetExt;
use gtk::{pango::EllipsizeMode, Label};

pub struct Description {
    gobject: Label,
}

impl Description {
    // Construct
    pub fn new() -> Self {
        let gobject = Label::builder()
            .css_classes(["subtitle"])
            .single_line_mode(true)
            .ellipsize(EllipsizeMode::End)
            .visible(false)
            .build();

        Self { gobject }
    }

    // Actions
    pub fn update(&self, text: Option<GString>) {
        match text {
            Some(value) => self.gobject.set_text(&value),
            None => self.gobject.set_text(""), // @TODO
        };
        self.gobject.set_visible(!self.gobject.text().is_empty());
    }

    // Getters
    pub fn gobject(&self) -> &Label {
        &self.gobject
    }
}
