mod ansi;
mod syntax;

use gtk::{
    TextBuffer, TextSearchFlags, TextTag, WrapMode,
    glib::{GString, uuid_string_random},
    prelude::{TextBufferExt, TextBufferExtManual},
};
use regex::Regex;
use std::collections::HashMap;
use syntax::Syntax;

const REGEX_CODE: &str = r"(?s)```[ \t]*(?P<alt>.*?)\n(?P<data>.*?)```";

// same with pre
// pub const ESCAPES: &[&str] = &["\\`"];

struct Entry {
    alt: Option<String>,
    data: String,
}

pub struct Code {
    index: HashMap<GString, Entry>,
    alt: TextTag,
}

impl Code {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            alt: TextTag::builder()
                .pixels_above_lines(4)
                .pixels_below_lines(8)
                .weight(500)
                .wrap_mode(WrapMode::None)
                .build(),
        }
    }

    /// Collect all code blocks into `Self.index` (to prevent formatting)
    pub fn collect(&mut self, buffer: &TextBuffer) {
        let (start, end) = buffer.bounds();
        let full_content = buffer.text(&start, &end, true).to_string();

        let matches: Vec<_> = Regex::new(REGEX_CODE)
            .unwrap()
            .captures_iter(&full_content)
            .collect();

        for cap in matches.into_iter().rev() {
            let id = uuid_string_random();

            let full_match = cap.get(0).unwrap();

            let start_char_offset = full_content[..full_match.start()].chars().count() as i32;
            let end_char_offset = full_content[..full_match.end()].chars().count() as i32;

            let mut start_iter = buffer.iter_at_offset(start_char_offset);
            let mut end_iter = buffer.iter_at_offset(end_char_offset);

            buffer.delete(&mut start_iter, &mut end_iter);

            buffer.insert_with_tags(&mut start_iter, &id, &[]);
            assert!(
                self.index
                    .insert(
                        id,
                        Entry {
                            alt: alt(cap["alt"].into()).map(|s| s.into()),
                            data: cap["data"].into(),
                        },
                    )
                    .is_none()
            )
        }
    }

    /// Apply code `Tag` to given `TextBuffer` using `Self.index`
    pub fn render(&mut self, buffer: &TextBuffer) {
        let syntax = Syntax::new();
        assert!(buffer.tag_table().add(&self.alt));
        for (k, v) in self.index.iter() {
            while let Some((mut m_start, mut m_end)) =
                buffer
                    .start_iter()
                    .forward_search(k, TextSearchFlags::VISIBLE_ONLY, None)
            {
                buffer.delete(&mut m_start, &mut m_end);
                if let Some(ref alt) = v.alt {
                    buffer.insert_with_tags(&mut m_start, &format!("{alt}\n"), &[&self.alt])
                }
                match syntax.highlight(&v.data, v.alt.as_ref()) {
                    Ok(highlight) => {
                        for (syntax_tag, entity) in highlight {
                            assert!(buffer.tag_table().add(&syntax_tag));
                            buffer.insert_with_tags(&mut m_start, &entity, &[&syntax_tag])
                        }
                    }
                    Err(_) => {
                        // Try ANSI/SGR format (terminal emulation) @TODO optional
                        for (syntax_tag, entity) in ansi::format(&v.data) {
                            assert!(buffer.tag_table().add(&syntax_tag));
                            buffer.insert_with_tags(&mut m_start, &entity, &[&syntax_tag])
                        }
                    }
                }
            }
        }
    }
}

fn alt(value: Option<&str>) -> Option<&str> {
    value.map(|m| m.trim()).filter(|s| !s.is_empty())
}

#[test]
fn test_regex() {
    let cap: Vec<_> = Regex::new(REGEX_CODE)
        .unwrap()
        .captures_iter("Some ``` alt text\ncode line 1\ncode line 2``` and ```\ncode line 3\ncode line 4``` with ![img](https://link.com)")
        .collect();

    let first = cap.first().unwrap();
    assert_eq!(alt(first.name("alt").map(|m| m.as_str())), Some("alt text"));
    assert_eq!(&first["data"], "code line 1\ncode line 2");

    let second = cap.get(1).unwrap();
    assert_eq!(alt(second.name("alt").map(|m| m.as_str())), None);
    assert_eq!(&second["data"], "code line 3\ncode line 4");
}
