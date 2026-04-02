use gtk::{
    TextBuffer, TextTag,
    WrapMode::Word,
    pango::Style,
    prelude::{TextBufferExt, TextBufferExtManual},
};
use regex::Regex;

const REGEX_ITALIC_1: &str = r"\*(?P<text>[^\*]*)\*";
const REGEX_ITALIC_2: &str = r"\b_(?P<text>[^_]*)_\b";

pub struct Italic(TextTag);

impl Italic {
    pub fn new() -> Self {
        Self(
            TextTag::builder()
                .style(Style::Italic)
                .wrap_mode(Word)
                .build(),
        )
    }

    /// Apply *italic*/_italic_ `Tag` to given `TextBuffer`
    /// * run after `Bold` tag!
    pub fn render(&self, buffer: &TextBuffer) {
        assert!(buffer.tag_table().add(&self.0));

        render(self, buffer, REGEX_ITALIC_1);
        render(self, buffer, REGEX_ITALIC_2);
    }
}

fn render(this: &Italic, buffer: &TextBuffer, regex: &str) {
    let (start, end) = buffer.bounds();
    let full_content = buffer.text(&start, &end, true).to_string();

    let matches: Vec<_> = Regex::new(regex)
        .unwrap()
        .captures_iter(&full_content)
        .collect();

    for cap in matches.into_iter().rev() {
        let full_match = cap.get(0).unwrap();

        let start_char_offset = full_content[..full_match.start()].chars().count() as i32;
        let end_char_offset = full_content[..full_match.end()].chars().count() as i32;

        let mut start_iter = buffer.iter_at_offset(start_char_offset);
        let mut end_iter = buffer.iter_at_offset(end_char_offset);

        if start_char_offset > 0
            && buffer
                .text(
                    &buffer.iter_at_offset(start_char_offset - 1),
                    &end_iter,
                    false,
                )
                .starts_with("\\")
        {
            continue;
        }

        let mut tags = start_iter.tags();
        tags.push(this.0.clone());

        buffer.delete(&mut start_iter, &mut end_iter);
        buffer.insert_with_tags(
            &mut start_iter,
            &cap["text"],
            &tags.iter().collect::<Vec<&TextTag>>(),
        )
    }
}

/// * run after `Bold` tag!
pub fn strip_tags(value: &str) -> String {
    let mut s = String::from(value);
    for cap in Regex::new(REGEX_ITALIC_1).unwrap().captures_iter(value) {
        if let Some(m) = cap.get(0) {
            s = s.replace(m.as_str(), &cap["text"]);
        }
    }
    for cap in Regex::new(REGEX_ITALIC_2).unwrap().captures_iter(value) {
        if let Some(m) = cap.get(0) {
            s = s.replace(m.as_str(), &cap["text"]);
        }
    }
    s
}

#[test]
fn test_strip_tags() {
    const S: &str = "Some *italic 1*\nand *italic 2* and _italic 3_";
    {
        let mut result = String::from(S);
        for cap in Regex::new(REGEX_ITALIC_1).unwrap().captures_iter(S) {
            if let Some(m) = cap.get(0) {
                result = result.replace(m.as_str(), &cap["text"]);
            }
        }
        assert_eq!(result, "Some italic 1\nand italic 2 and _italic 3_")
    }
    {
        let mut result = String::from(S);
        for cap in Regex::new(REGEX_ITALIC_2).unwrap().captures_iter(S) {
            if let Some(m) = cap.get(0) {
                result = result.replace(m.as_str(), &cap["text"]);
            }
        }
        assert_eq!(result, "Some *italic 1*\nand *italic 2* and italic 3")
    }
}

#[test]
fn test_regex() {
    const S: &str = "Some *italic 1*\nand *italic 2* and _italic 3_";
    {
        let cap: Vec<_> = Regex::new(REGEX_ITALIC_1)
            .unwrap()
            .captures_iter(S)
            .collect();

        assert_eq!(cap.len(), 2);

        let mut c = cap.into_iter();
        assert_eq!(&c.next().unwrap()["text"], "italic 1");
        assert_eq!(&c.next().unwrap()["text"], "italic 2");
    }
    {
        let cap: Vec<_> = Regex::new(REGEX_ITALIC_2)
            .unwrap()
            .captures_iter(S)
            .collect();

        assert_eq!(cap.len(), 1);

        let mut c = cap.into_iter();
        assert_eq!(&c.next().unwrap()["text"], "italic 3");
    }
}
