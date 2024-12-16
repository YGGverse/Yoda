mod current;
mod found;

use gtk::{TextTag, TextTagTable};

pub struct Tag {
    pub current: TextTag,
    pub found: TextTag,
}

impl Tag {
    // Constructors

    /// Create new `Self`
    pub fn new(table: TextTagTable) -> Self {
        // Init components
        let current = current::new();
        let found = found::new();

        // Init tag table
        // keep order as `current` should overwrite `found` tag style
        // https://docs.gtk.org/gtk4/method.TextTag.set_priority.html
        for &tag in &[&current, &found] {
            if !table.add(tag) {
                todo!()
            }
        }

        // Init `Self`
        Self { current, found }
    }
}
