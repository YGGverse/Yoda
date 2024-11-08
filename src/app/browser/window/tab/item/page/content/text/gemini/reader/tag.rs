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
    gobject: TextTagTable,
    // Tags
    code: Code,
    h1: H1,
    h2: H2,
    h3: H3,
    list: List,
    quote: Quote,
    title: Title,
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
        let gobject = TextTagTable::new();

        gobject.add(code.gobject());
        gobject.add(h1.gobject());
        gobject.add(h2.gobject());
        gobject.add(h3.gobject());
        gobject.add(title.gobject());
        gobject.add(list.gobject());
        gobject.add(quote.gobject());

        Self {
            gobject,
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
    pub fn add(&self, tag: &TextTag) -> bool {
        self.gobject.add(tag)
    }

    // Getters
    pub fn gobject(&self) -> &TextTagTable {
        &self.gobject
    }

    pub fn code(&self) -> &TextTag {
        self.code.gobject()
    }

    pub fn h1(&self) -> &TextTag {
        self.h1.gobject()
    }

    pub fn h2(&self) -> &TextTag {
        self.h2.gobject()
    }

    pub fn h3(&self) -> &TextTag {
        self.h3.gobject()
    }

    pub fn list(&self) -> &TextTag {
        self.list.gobject()
    }

    pub fn quote(&self) -> &TextTag {
        self.quote.gobject()
    }

    pub fn title(&self) -> &TextTag {
        self.title.gobject()
    }
}
