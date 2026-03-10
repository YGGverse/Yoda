mod bold;
mod code;
mod header;
mod list;
mod pre;
mod quote;
mod reference;
mod strike;
mod underline;

use bold::Bold;
use code::Code;
use gtk::{TextBuffer, TextSearchFlags, TextTag, gdk::RGBA, glib::Uri, prelude::TextBufferExt};
use header::Header;
use pre::Pre;
use quote::Quote;
use std::collections::HashMap;
use strike::Strike;
use underline::Underline;

pub struct Tags {
    pub bold: Bold,
    pub code: Code,
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
            code: Code::new(),
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
        // Collect all code blocks first,
        // and temporarily replace them with placeholder ID
        self.code.collect(buffer);

        // Keep in order!
        let title = self.header.render(buffer);

        list::render(buffer);

        self.quote.render(buffer);

        self.bold.render(buffer);
        self.pre.render(buffer);
        self.strike.render(buffer);
        self.underline.render(buffer);

        reference::render_images_links(buffer, base, link_color, links);
        reference::render_images(buffer, base, link_color, links);
        reference::render_links(buffer, base, link_color, links);

        // Cleanup unformatted escape chars
        let mut cursor = buffer.start_iter();
        while let Some((mut match_start, mut match_end)) =
            cursor.forward_search(ESC, TextSearchFlags::CASE_INSENSITIVE, None)
        {
            buffer.delete(&mut match_start, &mut match_end);
            cursor = match_end;
        }

        // Render placeholders
        self.code.render(buffer);

        // Format document title string
        title.map(|mut s| {
            s = bold::strip_tags(&s);
            s = pre::strip_tags(&s);
            s = reference::strip_tags(&s);
            s = strike::strip_tags(&s);
            s = underline::strip_tags(&s);
            s.replace(ESC, "")
        })
    }
}

const ESC: &str = "\\";
