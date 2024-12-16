mod current;
mod found;

use gtk::{TextTag, TextTagTable};

pub struct Tag {
    // pub current: TextTag,
    pub found: TextTag,
}

impl Tag {
    // Constructors

    pub fn new(tag_table: TextTagTable) -> Self {
        // Init components
        let current = current::new();
        let found = found::new();

        // Init `Self`
        tag_table.add(&current);
        tag_table.add(&found);

        Self {
            /*current,*/ found,
        }
    }
}
