use gtk::Entry;
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

    // Getters
    pub fn gobject(&self) -> &Entry {
        &self.gobject
    }
}
