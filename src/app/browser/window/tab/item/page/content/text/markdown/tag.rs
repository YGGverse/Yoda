mod header;
mod list;
mod plain;
mod quote;
mod title;

use gtk::{
    TextBuffer, TextTag, TextTagTable,
    prelude::{TextBufferExt, TextBufferExtManual},
};
use header::Header;
use list::List;
use plain::Plain;
use quote::Quote;
use regex::Regex;
use title::Title;

pub struct Tag {
    pub text_tag_table: TextTagTable,
    // Tags
    pub h1: TextTag,
    pub h2: TextTag,
    pub h3: TextTag,
    pub h4: TextTag,
    pub h5: TextTag,
    pub h6: TextTag,
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
        let h1 = TextTag::h1();
        let h2 = TextTag::h2();
        let h3 = TextTag::h3();
        let h4 = TextTag::h4();
        let h5 = TextTag::h5();
        let h6 = TextTag::h6();
        let list = TextTag::list();
        let quote = TextTag::quote();
        let title = TextTag::title();
        let plain = TextTag::plain();

        // Init tag table
        let text_tag_table = TextTagTable::new();

        text_tag_table.add(&h1);
        text_tag_table.add(&h2);
        text_tag_table.add(&h3);
        text_tag_table.add(&h4);
        text_tag_table.add(&h5);
        text_tag_table.add(&h6);
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
            h4,
            h5,
            h6,
            list,
            quote,
            title,
            plain,
        }
    }
}

// Headers `#`, `##`, etc.

const REGEX_HEADER: &str = r"(?m)^(?P<level>#{1,6})\s+(?P<title>.*)$";

/// Apply header `Tag` to given `TextBuffer`
pub fn header(buffer: &TextBuffer, tag: &Tag) {
    let (start, end) = buffer.bounds();
    let full_content = buffer.text(&start, &end, true).to_string();

    let matches: Vec<_> = Regex::new(REGEX_HEADER)
        .unwrap()
        .captures_iter(&full_content)
        .collect();

    for cap in matches.into_iter().rev() {
        let full_match = cap.get(0).unwrap();

        let start_char_offset = full_content[..full_match.start()].chars().count() as i32;
        let end_char_offset = full_content[..full_match.end()].chars().count() as i32;

        let mut start_iter = buffer.iter_at_offset(start_char_offset);
        let mut end_iter = buffer.iter_at_offset(end_char_offset);

        buffer.delete(&mut start_iter, &mut end_iter);

        match cap["level"].chars().count() {
            1 => buffer.insert_with_tags(&mut start_iter, &cap["title"], &[&tag.h1]),
            2 => buffer.insert_with_tags(&mut start_iter, &cap["title"], &[&tag.h2]),
            3 => buffer.insert_with_tags(&mut start_iter, &cap["title"], &[&tag.h3]),
            4 => buffer.insert_with_tags(&mut start_iter, &cap["title"], &[&tag.h4]),
            5 => buffer.insert_with_tags(&mut start_iter, &cap["title"], &[&tag.h5]),
            6 => buffer.insert_with_tags(&mut start_iter, &cap["title"], &[&tag.h6]),
            _ => buffer.insert_with_tags(&mut start_iter, &cap["title"], &[]),
        }
    }
}

#[test]
fn test_regex_header() {
    let cap: Vec<_> = Regex::new(REGEX_HEADER)
        .unwrap()
        .captures_iter(r"## Title ![alt](https://link.com)")
        .collect();

    let first = cap.get(0).unwrap();
    assert_eq!(&first[0], "## Title ![alt](https://link.com)");
    assert_eq!(&first["level"], "##");
    assert_eq!(&first["title"], "Title ![alt](https://link.com)");
}

// Quotes

const REGEX_QUOTE: &str = r"(?m)^>\s+(?P<text>.*)$";

/// Apply quote `Tag` to given `TextBuffer`
pub fn quote(buffer: &TextBuffer, tag: &Tag) {
    let (start, end) = buffer.bounds();
    let full_content = buffer.text(&start, &end, true).to_string();

    let matches: Vec<_> = Regex::new(REGEX_QUOTE)
        .unwrap()
        .captures_iter(&full_content)
        .collect();

    for cap in matches.into_iter().rev() {
        let full_match = cap.get(0).unwrap();

        let start_char_offset = full_content[..full_match.start()].chars().count() as i32;
        let end_char_offset = full_content[..full_match.end()].chars().count() as i32;

        let mut start_iter = buffer.iter_at_offset(start_char_offset);
        let mut end_iter = buffer.iter_at_offset(end_char_offset);

        buffer.delete(&mut start_iter, &mut end_iter);
        buffer.insert_with_tags(&mut start_iter, &cap["text"], &[&tag.quote])
    }
}

#[test]
fn test_regex_quote() {
    let cap: Vec<_> = Regex::new(REGEX_QUOTE)
        .unwrap()
        .captures_iter(r"> Some quote with ![img](https://link.com)")
        .collect();

    let first = cap.get(0).unwrap();
    assert_eq!(&first[0], "> Some quote with ![img](https://link.com)");
    assert_eq!(&first["text"], "Some quote with ![img](https://link.com)");
}
