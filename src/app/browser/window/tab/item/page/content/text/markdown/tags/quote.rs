use gtk::{
    TextBuffer, TextTag,
    WrapMode::Word,
    pango::Style::Italic,
    prelude::{TextBufferExt, TextBufferExtManual},
};
use regex::Regex;

const REGEX_QUOTE: &str = r"(?m)^>(?:[ \t]*(?P<text>.*))?$";

pub struct Quote(TextTag);

impl Quote {
    pub fn new() -> Self {
        Self(
            TextTag::builder()
                .left_margin(28)
                .wrap_mode(Word)
                .style(Italic) // conflicts the italic tags decoration @TODO
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

            buffer.delete(&mut start_iter, &mut end_iter);
            buffer.insert_with_tags(&mut start_iter, &cap["text"], &[&self.0])
        }
    }
}

#[test]
fn test_regex() {
    let cap: Vec<_> = Regex::new(REGEX_QUOTE).unwrap().captures_iter(
        "> Some quote 1 with ![img](https://link.com)\n>\n> 2\\)Some quote 2 with text\nplain text\n> Some quote 3"
    ).collect();

    let mut i = cap.into_iter();

    assert_eq!(
        &i.next().unwrap()["text"],
        "Some quote 1 with ![img](https://link.com)"
    );
    assert!(&i.next().unwrap()["text"].is_empty());
    assert_eq!(&i.next().unwrap()["text"], "2\\)Some quote 2 with text");
    assert_eq!(&i.next().unwrap()["text"], "Some quote 3");
}
