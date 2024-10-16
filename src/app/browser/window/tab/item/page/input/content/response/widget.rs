use gtk::{
    prelude::{EditableExt, EntryExt, WidgetExt},
    Entry,
};
use std::sync::Arc;

pub struct Widget {
    gobject: Entry,
}

impl Widget {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        let gobject = Entry::builder().hexpand(true).build();

        Arc::new(Self { gobject })
    }

    // Actions
    pub fn set(&self, placeholder_text: &str, sensitive: bool) {
        self.gobject.set_text(&""); // reset
        self.gobject.set_placeholder_text(Some(placeholder_text));
        self.gobject.set_sensitive(sensitive);
    }

    // Getters
    pub fn gobject(&self) -> &Entry {
        &self.gobject
    }
}
