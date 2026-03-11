use gtk::{
    TextBuffer, TextTag,
    WrapMode::Word,
    gdk::RGBA,
    prelude::{TextBufferExt, TextBufferExtManual},
};
use regex::Regex;

const REGEX_PRE: &str = r"`(?P<text>[^`]+)`";
const TAG_FONT: &str = "monospace"; // @TODO
const TAG_SCALE: f64 = 0.9;

pub struct Pre(TextTag);

impl Pre {
    pub fn new() -> Self {
        Self(if adw::StyleManager::default().is_dark() {
            TextTag::builder()
                .background_rgba(&RGBA::new(255., 255., 255., 0.05))
                .family(TAG_FONT)
                .foreground("#e8e8e8")
                .scale(TAG_SCALE)
                .wrap_mode(Word)
                .build()
        } else {
            TextTag::builder()
                .background_rgba(&RGBA::new(0., 0., 0., 0.06))
                .family(TAG_FONT)
                .scale(TAG_SCALE)
                .wrap_mode(Word)
                .build()
        })
    }

    /// Apply preformatted `Tag` to given `TextBuffer`
    pub fn render(&self, buffer: &TextBuffer) {
        assert!(buffer.tag_table().add(&self.0));

        let (start, end) = buffer.bounds();
        let full_content = buffer.text(&start, &end, true).to_string();

        let matches: Vec<_> = Regex::new(REGEX_PRE)
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

            buffer.delete(&mut start_iter, &mut end_iter);
            buffer.insert_with_tags(&mut start_iter, &cap["text"], &[&self.0])
        }
    }
}

pub fn strip_tags(value: &str) -> String {
    let mut result = String::from(value);
    for cap in Regex::new(REGEX_PRE).unwrap().captures_iter(value) {
        if let Some(m) = cap.get(0) {
            result = result.replace(m.as_str(), &cap["text"]);
        }
    }
    result
}

#[test]
fn test_strip_tags() {
    const VALUE: &str = r"Some `pre 1` and `pre 2` with ![img](https://link.com)";
    let mut result = String::from(VALUE);
    for cap in Regex::new(REGEX_PRE).unwrap().captures_iter(VALUE) {
        if let Some(m) = cap.get(0) {
            result = result.replace(m.as_str(), &cap["text"]);
        }
    }
    assert_eq!(result, "Some pre 1 and pre 2 with ![img](https://link.com)")
}

#[test]
fn test_regex() {
    let cap: Vec<_> = Regex::new(REGEX_PRE)
        .unwrap()
        .captures_iter(r"Some `pre 1` and `pre 2` with ![img](https://link.com)")
        .collect();

    assert_eq!(&cap.first().unwrap()["text"], "pre 1");
    assert_eq!(&cap.get(1).unwrap()["text"], "pre 2");
}
