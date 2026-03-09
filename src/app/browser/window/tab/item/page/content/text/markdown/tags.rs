mod bold;
mod header;
mod pre;
mod quote;
mod reference;
mod strike;
mod underline;

use std::collections::HashMap;

use bold::Bold;
use gtk::{TextBuffer, TextTag, gdk::RGBA, glib::Uri};
use header::Header;
use pre::Pre;
use quote::Quote;
use strike::Strike;
use underline::Underline;

pub struct Tags {
    pub bold: Bold,
    pub header: Header,
    pub pre: Pre,
    pub quote: Quote,
    pub strike: Strike,
    pub underline: Underline,
}

impl Default for Tags {
    fn default() -> Self {
        Self::new()
    }
}

impl Tags {
    // Construct
    pub fn new() -> Self {
        Self {
            bold: Bold::new(),
            header: Header::new(),
            pre: Pre::new(),
            quote: Quote::new(),
            strike: Strike::new(),
            underline: Underline::new(),
        }
    }
    pub fn render(
        &mut self,
        buffer: &TextBuffer,
        base: &Uri,
        link_color: &RGBA,
        links: &mut HashMap<TextTag, Uri>,
    ) -> Option<String> {
        // Collect all preformatted blocks first, and replace them with tmp macro ID
        self.pre.collect(buffer);

        // Keep in order!
        let title = self.header.render(buffer);

        self.quote.render(buffer);

        self.bold.render(buffer);
        self.strike.render(buffer);
        self.underline.render(buffer);

        reference::render_images_links(buffer, base, link_color, links);
        reference::render_images(buffer, base, link_color, links);
        reference::render_links(buffer, base, link_color, links);

        self.pre.render(buffer);

        // Format document title string
        title.map(|mut s| {
            s = bold::strip_tags(&s);
            s = reference::strip_tags(&s);
            s = strike::strip_tags(&s);
            s = underline::strip_tags(&s);
            s // @TODO other tags
        })
    }
}
