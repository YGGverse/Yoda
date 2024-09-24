use gtk::{prelude::EntryExt, Entry};

pub struct Request {
    widget: Entry,
}

impl Request {
    // Construct
    pub fn new() -> Request {
        Self {
            widget: Entry::builder()
                .placeholder_text("URL or search term...")
                .hexpand(true)
                .progress_pulse_step(0.1)
                .build(),
        }
    }

    // Actions
    pub fn update(&self) {
        // @TODO
    }

    // Getters
    pub fn widget(&self) -> &Entry {
        &self.widget
    }

    pub fn is_empty(&self) -> bool {
        0 == self.widget.text_length()
    }
}
