mod header;
mod list;
mod plain;
mod quote;
mod reference;
mod title;

use std::collections::HashMap;

use gtk::{
    TextBuffer, TextTag, TextTagTable,
    gdk::RGBA,
    glib::Uri,
    prelude::{TextBufferExt, TextBufferExtManual},
};
use header::Header;
use list::List;
use plain::Plain;
use quote::Quote;
use reference::Reference;
use title::Title;

pub struct Tags {
    pub text_tag_table: TextTagTable,
    // Tags
    pub header: Header,
    pub list: TextTag,
    pub plain: TextTag,
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

        // Init components
        let list = TextTag::list();
        let plain = TextTag::plain();
        let title = TextTag::title();
        text_tag_table.add(&title);
        text_tag_table.add(&list);
        text_tag_table.add(&plain);

        Self {
            text_tag_table,
            // Tags
            header: Header::new(),
            list,
            plain,
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
    ) {
        // * keep in order!
        self.header.render(buffer);
        self.quote.render(buffer);

        reference::render_images_links(&buffer, base, &link_color, links);
        reference::render_images(&buffer, base, &link_color, links);
        reference::render_links(&buffer, base, &link_color, links);
    }
}
