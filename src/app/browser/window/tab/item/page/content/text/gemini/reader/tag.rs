mod h1;
mod h2;
mod h3;
mod list;
mod plain;
mod quote;
mod title;

use gtk::{TextTag, TextTagTable};

pub struct Tag {
    pub text_tag_table: TextTagTable,
    // Tags
    pub h1: TextTag,
    pub h2: TextTag,
    pub h3: TextTag,
    pub list: TextTag,
    pub quote: TextTag,
    pub title: TextTag,
    pub plain: TextTag,
}

impl Default for Tag {
    fn default() -> Self {
        Self::new()
    }
}

impl Tag {
    // Construct
    pub fn new() -> Self {
        // Init components
        let h1 = h1::new();
        let h2 = h2::new();
        let h3 = h3::new();
        let list = list::new();
        let quote = quote::new();
        let title = title::new();
        let plain = plain::new();

        // Init tag table
        let text_tag_table = TextTagTable::new();

        text_tag_table.add(&h1);
        text_tag_table.add(&h2);
        text_tag_table.add(&h3);
        text_tag_table.add(&title);
        text_tag_table.add(&list);
        text_tag_table.add(&quote);
        text_tag_table.add(&plain);

        Self {
            text_tag_table,
            // Tags
            h1,
            h2,
            h3,
            list,
            quote,
            title,
            plain,
        }
    }
}
