use gtk::{
    TextBuffer, TextTag, WrapMode,
    prelude::{TextBufferExt, TextBufferExtManual},
};
use regex::Regex;

const REGEX_HEADER: &str = r"(?m)^(?P<level>#{1,6})\s+(?P<title>.*)$";

pub struct Header {
    h1: TextTag,
    h2: TextTag,
    h3: TextTag,
    h4: TextTag,
    h5: TextTag,
    h6: TextTag,
}

impl Header {
    pub fn new() -> Self {
        // * important to give the tag name here as used in the fragment search
        Self {
            h1: TextTag::builder()
                .foreground("#2190a4") // @TODO optional
                .name("h1")
                .scale(1.6)
                .sentence(true)
                .weight(500)
                .wrap_mode(WrapMode::Word)
                .build(),
            h2: TextTag::builder()
                .foreground("#d56199") // @TODO optional
                .name("h2")
                .scale(1.4)
                .sentence(true)
                .weight(400)
                .wrap_mode(WrapMode::Word)
                .build(),
            h3: TextTag::builder()
                .foreground("#c88800") // @TODO optional
                .name("h3")
                .scale(1.2)
                .sentence(true)
                .weight(400)
                .wrap_mode(WrapMode::Word)
                .build(),
            h4: TextTag::builder()
                .foreground("#c88800") // @TODO optional
                .name("h4")
                .scale(1.1)
                .sentence(true)
                .weight(400)
                .wrap_mode(WrapMode::Word)
                .build(),
            h5: TextTag::builder()
                .foreground("#c88800") // @TODO optional
                .name("h5")
                .scale(1.0)
                .sentence(true)
                .weight(400)
                .wrap_mode(WrapMode::Word)
                .build(),
            h6: TextTag::builder()
                .foreground("#c88800") // @TODO optional
                .name("h6")
                .scale(1.0)
                .sentence(true)
                .weight(300)
                .wrap_mode(WrapMode::Word)
                .build(),
        }
    }

    /// Apply title `Tag` to given `TextBuffer`
    pub fn render(&self, buffer: &TextBuffer) -> Option<String> {
        let mut raw_title = None;

        let table = buffer.tag_table();

        assert!(table.add(&self.h1));
        assert!(table.add(&self.h2));
        assert!(table.add(&self.h3));
        assert!(table.add(&self.h4));
        assert!(table.add(&self.h5));

        let (start, end) = buffer.bounds();
        let full_content = buffer.text(&start, &end, true).to_string();

        let matches: Vec<_> = Regex::new(REGEX_HEADER)
            .unwrap()
            .captures_iter(&full_content)
            .collect();

        for cap in matches.iter() {
            if raw_title.is_none() && !cap["title"].trim().is_empty() {
                raw_title = Some(cap["title"].into())
            }
        }

        for cap in matches.into_iter().rev() {
            let full_match = cap.get(0).unwrap();

            let start_char_offset = full_content[..full_match.start()].chars().count() as i32;
            let end_char_offset = full_content[..full_match.end()].chars().count() as i32;

            let mut start_iter = buffer.iter_at_offset(start_char_offset);
            let mut end_iter = buffer.iter_at_offset(end_char_offset);

            buffer.delete(&mut start_iter, &mut end_iter);

            match cap["level"].chars().count() {
                1 => buffer.insert_with_tags(&mut start_iter, &cap["title"], &[&self.h1]),
                2 => buffer.insert_with_tags(&mut start_iter, &cap["title"], &[&self.h2]),
                3 => buffer.insert_with_tags(&mut start_iter, &cap["title"], &[&self.h3]),
                4 => buffer.insert_with_tags(&mut start_iter, &cap["title"], &[&self.h4]),
                5 => buffer.insert_with_tags(&mut start_iter, &cap["title"], &[&self.h5]),
                6 => buffer.insert_with_tags(&mut start_iter, &cap["title"], &[&self.h6]),
                _ => buffer.insert_with_tags(&mut start_iter, &cap["title"], &[]),
            }
        }

        raw_title
    }
}

#[test]
fn test_regex_title() {
    let cap: Vec<_> = Regex::new(REGEX_HEADER)
        .unwrap()
        .captures_iter(r"## Header ![alt](https://link.com)")
        .collect();

    let first = cap.first().unwrap();
    assert_eq!(&first[0], "## Header ![alt](https://link.com)");
    assert_eq!(&first["level"], "##");
    assert_eq!(&first["title"], "Header ![alt](https://link.com)");
}
