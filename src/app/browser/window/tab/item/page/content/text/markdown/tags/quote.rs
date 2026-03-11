use gtk::{
    TextBuffer, TextTag,
    WrapMode::Word,
    pango::Style::Italic,
    prelude::{TextBufferExt, TextBufferExtManual},
};
use regex::Regex;

const REGEX_QUOTE: &str = r"(?m)>\s*(?P<text>.*)$";

pub const ESCAPES: &[&str] = &["\\>"];

pub struct Quote(TextTag);

impl Quote {
    pub fn new() -> Self {
        Self(
            TextTag::builder()
                .left_margin(28)
                .wrap_mode(Word)
                .style(Italic) // what about the italic tags decoration? @TODO
                .build(),
        )
    }

    /// Apply quote `Tag` to given `TextBuffer`
    pub fn render(&self, buffer: &TextBuffer) {
        assert!(buffer.tag_table().add(&self.0));

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

#[test]
fn test_regex() {
    let cap: Vec<_> = Regex::new(REGEX_QUOTE).unwrap().captures_iter(
        "> Some quote 1 with ![img](https://link.com)\n> Some quote 2 with text\nplain text\n> Some quote 3"
    ).collect();
    {
        let m = cap.first().unwrap();
        assert_eq!(&m["text"], "Some quote 1 with ![img](https://link.com)");
    }
    {
        let m = cap.get(1).unwrap();
        assert_eq!(&m["text"], "Some quote 2 with text");
    }
    {
        let m = cap.get(2).unwrap();
        assert_eq!(&m["text"], "Some quote 3");
    }
}
