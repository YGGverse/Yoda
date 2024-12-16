mod current;
mod found;

use gtk::{TextTag, TextTagTable};

pub struct Tag {
    pub current: TextTag,
    pub found: TextTag,
}

impl Tag {
    // Constructors

    pub fn new(tag_table: TextTagTable) -> Self {
        // Init components
        let current = current::new();
        let found = found::new();

        // Init `Self`
        tag_table.add(&found);
        tag_table.add(&current); // keep current priority as `current` should overwrite `found`
                                 // https://docs.gtk.org/gtk4/method.TextTag.set_priority.html

        Self { current, found }
    }
}
