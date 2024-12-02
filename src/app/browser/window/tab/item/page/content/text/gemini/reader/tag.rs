mod code;
mod h1;
mod h2;
mod h3;
mod list;
mod quote;
mod title;

use code::Code;
use h1::H1;
use h2::H2;
use h3::H3;
use list::List;
use quote::Quote;
use title::Title;

use gtk::{TextTag, TextTagTable};

pub struct Tag {
    pub text_tag_table: TextTagTable,
    // Tags
    pub code: Code,
    pub h1: H1,
    pub h2: H2,
    pub h3: H3,
    pub list: List,
    pub quote: Quote,
    pub title: Title,
}

impl Tag {
    // Construct
    pub fn new() -> Self {
        // Init components
        let code = Code::new();
        let h1 = H1::new();
        let h2 = H2::new();
        let h3 = H3::new();
        let list = List::new();
        let quote = Quote::new();
        let title = Title::new();

        // Init tag table
        let text_tag_table = TextTagTable::new();

        text_tag_table.add(&code.text_tag);
        text_tag_table.add(&h1.text_tag);
        text_tag_table.add(&h2.text_tag);
        text_tag_table.add(&h3.text_tag);
        text_tag_table.add(&title.text_tag);
        text_tag_table.add(&list.text_tag);
        text_tag_table.add(&quote.text_tag);

        Self {
            text_tag_table,
            // Tags
            code,
            h1,
            h2,
            h3,
            list,
            quote,
            title,
        }
    }

    // Actions
    pub fn add(&self, text_tag: &TextTag) -> bool {
        self.text_tag_table.add(text_tag)
    }
}
