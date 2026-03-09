use gtk::{
    TextBuffer, TextSearchFlags, TextTag,
    WrapMode::Word,
    glib::{GString, uuid_string_random},
    prelude::{TextBufferExt, TextBufferExtManual},
};
use regex::Regex;
use std::collections::HashMap;

const REGEX_PRE: &str = r"(?s)```[ \t]*(?P<alt>.*?)\n(?P<data>.*?)```";

struct Entry {
    alt: Option<String>,
    data: String,
}

pub struct Pre {
    index: HashMap<GString, Entry>,
    tag: TextTag,
}

impl Pre {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            tag: TextTag::builder().wrap_mode(Word).build(), // @TODO
        }
    }

    /// Collect all preformatted blocks into `Self.index` (to prevent formatting)
    pub fn collect(&mut self, buffer: &TextBuffer) {
        let (start, end) = buffer.bounds();
        let full_content = buffer.text(&start, &end, true).to_string();

        let matches: Vec<_> = Regex::new(REGEX_PRE)
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

    /// Apply preformatted `Tag` to given `TextBuffer` using `Self.index`
    pub fn render(&mut self, buffer: &TextBuffer) {
        assert!(buffer.tag_table().add(&self.tag));
        for (k, v) in self.index.iter() {
            while let Some((mut m_start, mut m_end)) =
                buffer
                    .start_iter()
                    .forward_search(k, TextSearchFlags::VISIBLE_ONLY, None)
            {
                buffer.delete(&mut m_start, &mut m_end);

                let alt_text = v.alt.as_deref().unwrap_or("");
                let display_text = format!("{} |\n {}", alt_text, v.data);

                buffer.insert_with_tags(&mut m_start, &display_text, &[&self.tag]);
            }
        }
    }
}

fn alt(value: Option<&str>) -> Option<&str> {
    value.map(|m| m.trim()).filter(|s| !s.is_empty())
}

#[test]
fn test_regex() {
    let cap: Vec<_> = Regex::new(REGEX_PRE)
        .unwrap()
        .captures_iter("Some ``` alt text\ncode line 1\ncode line 2``` and ```\ncode line 3\ncode line 4``` with ![img](https://link.com)")
        .collect();

    let first = cap.get(0).unwrap();
    assert_eq!(alt(first.name("alt").map(|m| m.as_str())), Some("alt text"));
    assert_eq!(&first["data"], "code line 1\ncode line 2");

    let second = cap.get(1).unwrap();
    assert_eq!(alt(second.name("alt").map(|m| m.as_str())), None);
    assert_eq!(&second["data"], "code line 3\ncode line 4");
}
