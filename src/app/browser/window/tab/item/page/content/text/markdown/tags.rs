mod bold;
mod header;
mod list;
mod quote;
mod reference;
mod title;

use std::collections::HashMap;

use bold::Bold;
use gtk::{
    TextBuffer, TextTag, TextTagTable,
    gdk::RGBA,
    glib::Uri,
    prelude::{TextBufferExt, TextBufferExtManual},
};
use header::Header;
use list::List;
use quote::Quote;
use title::Title;

pub struct Tags {
    pub text_tag_table: TextTagTable,
    // Tags
    pub bold: Bold,
    pub header: Header,
    pub list: TextTag,
    pub quote: Quote,
    pub title: TextTag,
}

impl Default for Tags {
    fn default() -> Self {
        Self::new()
    }
}

impl Tags {
    // Construct
    pub fn new() -> Self {
        // Init tag table
        let text_tag_table = TextTagTable::new();

        // Init shared tags members
        let list = TextTag::list();
        let title = TextTag::title();
        text_tag_table.add(&title);
        text_tag_table.add(&list);

        Self {
            text_tag_table,
            // Tags
            bold: Bold::new(),
            header: Header::new(),
            list,
            quote: Quote::new(),
            title,
        }
    }
    pub fn render(
        &self,
        buffer: &TextBuffer,
        base: &Uri,
        link_color: &RGBA,
        links: &mut HashMap<TextTag, Uri>,
    ) -> Option<String> {
        // * keep in order!
        let title = self.header.render(buffer);

        self.quote.render(buffer);

        self.bold.render(buffer);

        reference::render_images_links(&buffer, base, &link_color, links);
        reference::render_images(&buffer, base, &link_color, links);
        reference::render_links(&buffer, base, &link_color, links);

        title.map(|mut s| {
            s = reference::strip_tags(&s);
            s = bold::strip_tags(&s);
            s // @TODO other tags
        })
    }
}
