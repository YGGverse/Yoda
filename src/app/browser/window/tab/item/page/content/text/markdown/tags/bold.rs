use gtk::{
    TextBuffer, TextTag,
    WrapMode::Word,
    prelude::{TextBufferExt, TextBufferExtManual},
};
use regex::Regex;

const REGEX_BOLD: &str = r"(\*\*|__)(?P<text>[^\*_]*)(\*\*|__)";

pub struct Bold(TextTag);

impl Bold {
    pub fn new() -> Self {
        Self(TextTag::builder().weight(600).wrap_mode(Word).build())
    }

    /// Apply **bold**/__bold__ `Tag` to given `TextBuffer`
    pub fn render(&self, buffer: &TextBuffer) {
        assert!(buffer.tag_table().add(&self.0));

        let (start, end) = buffer.bounds();
        let full_content = buffer.text(&start, &end, true).to_string();

        let matches: Vec<_> = Regex::new(REGEX_BOLD)
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
    for cap in Regex::new(REGEX_BOLD).unwrap().captures_iter(value) {
        if let Some(m) = cap.get(0) {
            result = result.replace(m.as_str(), &cap["text"]);
        }
    }
    result
}

#[test]
fn test_strip_tags() {
    const VALUE: &str =
        "Some **bold 1** and **bold 2** and __bold 3__ and *italic 1* and _italic 2_";
    let mut result = String::from(VALUE);
    for cap in Regex::new(REGEX_BOLD).unwrap().captures_iter(VALUE) {
        if let Some(m) = cap.get(0) {
            result = result.replace(m.as_str(), &cap["text"]);
        }
    }
    assert_eq!(
        result,
        "Some bold 1 and bold 2 and bold 3 and *italic 1* and _italic 2_"
    )
}

#[test]
fn test_regex() {
    let cap: Vec<_> = Regex::new(REGEX_BOLD)
        .unwrap()
        .captures_iter(
            "Some **bold 1** and **bold 2** and __bold 3__ and *italic 1* and _italic 2_",
        )
        .collect();

    assert_eq!(cap.len(), 3);

    let mut c = cap.into_iter();
    assert_eq!(&c.next().unwrap()["text"], "bold 1");
    assert_eq!(&c.next().unwrap()["text"], "bold 2");
    assert_eq!(&c.next().unwrap()["text"], "bold 3");
}
