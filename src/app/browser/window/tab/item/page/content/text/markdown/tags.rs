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
use gtk::{
    TextBuffer, TextSearchFlags, TextTag,
    gdk::RGBA,
    glib::{GString, Uri},
    prelude::TextBufferExt,
};
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
        headers: &mut HashMap<TextTag, (String, Uri)>,
    ) -> Option<String> {
        // Collect all code blocks first,
        // and temporarily replace them with placeholder ID
        self.code.collect(buffer);

        // Keep in order!
        let title = self.header.render(buffer, base, headers);

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
        for e in ESCAPE_ENTRIES {
            let mut cursor = buffer.start_iter();
            while let Some((mut match_start, mut match_end)) =
                cursor.forward_search(e, TextSearchFlags::CASE_INSENSITIVE, None)
            {
                if match_end.backward_cursor_positions(1) {
                    buffer.delete(&mut match_start, &mut match_end)
                }
                cursor = match_end;
            }
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
            for e in ESCAPE_ENTRIES {
                s = s.replace(e, &e[1..]);
            }
            s
        })
    }
}

/// Shared URL #fragment logic (for the Header tags ref)
pub fn format_header_fragment(value: &str) -> GString {
    Uri::escape_string(&value.to_lowercase().replace(" ", "-"), None, true)
}

const ESCAPE_ENTRIES: &[&str] = &[
    "\\\n", "\\\\", "\\>", "\\`", "\\!", "\\[", "\\]", "\\(", "\\)", "\\*", "\\#", "\\~", "\\_",
];
#[test]
fn test_escape_entries() {
    for e in ESCAPE_ENTRIES {
        assert_eq!(e.len(), 2)
    }
}
