use std::sync::Arc;

use gtk::{glib::GString, pango::EllipsizeMode, prelude::WidgetExt, Label};

const DEFAULT_LABEL_TEXT: &str = "New page";

pub struct Title {
    gobject: Label,
}

impl Title {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        Arc::new(Self {
            gobject: Label::builder()
                .label(DEFAULT_LABEL_TEXT)
                .ellipsize(EllipsizeMode::End)
                .width_chars(16)
                .single_line_mode(true)
                .build(),
        })
    }

    // Actions
    pub fn update(&self, title: Option<&GString>) {
        match title {
            Some(title) => self.gobject.set_text(title),
            None => self.gobject.set_text(DEFAULT_LABEL_TEXT),
        }
    }

    pub fn pin(&self, is_pinned: bool) {
        self.gobject.set_visible(!is_pinned);
    }

    // Getters
    pub fn gobject(&self) -> &Label {
        &self.gobject
    }
}
