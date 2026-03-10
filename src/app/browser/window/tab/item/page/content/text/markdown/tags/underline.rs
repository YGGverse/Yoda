use gtk::{
    TextBuffer, TextTag,
    WrapMode::Word,
    pango::Underline::Single,
    prelude::{TextBufferExt, TextBufferExtManual},
};
use regex::Regex;

const REGEX_UNDERLINE: &str = r"\b_(?P<text>[^_]+)_\b";

pub struct Underline(TextTag);

impl Underline {
    pub fn new() -> Self {
        Self(TextTag::builder().underline(Single).wrap_mode(Word).build())
    }

    /// Apply _underline_ `Tag` to given `TextBuffer`
    pub fn render(&self, buffer: &TextBuffer) {
        assert!(buffer.tag_table().add(&self.0));

        let (start, end) = buffer.bounds();
        let full_content = buffer.text(&start, &end, true).to_string();

        let matches: Vec<_> = Regex::new(REGEX_UNDERLINE)
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
                    .contains("\\")
            {
                continue;
            }

            let mut tags = start_iter.tags();
            tags.push(self.0.clone());

            buffer.delete(&mut start_iter, &mut end_iter);
            buffer.insert_with_tags(
                &mut start_iter,
                &cap["text"],
                &tags.iter().collect::<Vec<&TextTag>>(),
            )
        }
    }
}

pub fn strip_tags(value: &str) -> String {
    let mut result = String::from(value);
    for cap in Regex::new(REGEX_UNDERLINE).unwrap().captures_iter(value) {
        if let Some(m) = cap.get(0) {
            result = result.replace(m.as_str(), &cap["text"]);
        }
    }
    result
}

#[test]
fn test_strip_tags() {
    const VALUE: &str = r"Some _underline 1_ and _underline 2_ with ![img](https://link.com)";
    let mut result = String::from(VALUE);
    for cap in Regex::new(REGEX_UNDERLINE).unwrap().captures_iter(VALUE) {
        if let Some(m) = cap.get(0) {
            result = result.replace(m.as_str(), &cap["text"]);
        }
    }
    assert_eq!(
        result,
        "Some underline 1 and underline 2 with ![img](https://link.com)"
    )
}

#[test]
fn test_regex() {
    let cap: Vec<_> = Regex::new(REGEX_UNDERLINE)
        .unwrap()
        .captures_iter(r"Some _underline 1_ and _underline 2_ with ![img](https://link.com)")
        .collect();

    assert_eq!(&cap.first().unwrap()["text"], "underline 1");
    assert_eq!(&cap.get(1).unwrap()["text"], "underline 2");
}
