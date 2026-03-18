use gtk::{
    TextBuffer, TextSearchFlags, TextView,
    glib::{GString, uuid_string_random},
    prelude::{TextBufferExt, TextBufferExtManual, TextViewExt},
};
use regex::Regex;
use std::collections::HashMap;

const REGEX_CODE: &str = r"(?s)```[ \t]*(?P<alt>.*?)\n(?P<data>.*?)```";

struct Entry {
    alt: Option<String>,
    data: String,
}

pub struct Code(HashMap<GString, Entry>);

impl Code {
    pub fn new() -> Self {
        Self(HashMap::new())
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
                self.0
                    .insert(
                        id,
                        Entry {
                            alt: alt(cap["alt"].into()).map(|s| s.into()),
                            data: cap["data"].trim_end().into(),
                        },
                    )
                    .is_none()
            )
        }
    }

    /// Apply code `Tag` to given `TextView` using `Self.index`
    pub fn render(&mut self, text_view: &TextView) {
        let buffer = text_view.buffer();
        for (k, v) in self.0.iter() {
            while let Some((mut m_start, mut m_end)) =
                buffer
                    .start_iter()
                    .forward_search(k, TextSearchFlags::VISIBLE_ONLY, None)
            {
                buffer.delete(&mut m_start, &mut m_end);
                text_view.add_child_at_anchor(
                    &super::super::super::common::Code::init(
                        text_view,
                        v.data.as_ref(),
                        v.alt.as_ref(),
                    )
                    .widget,
                    &buffer.create_child_anchor(&mut m_end),
                )
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
