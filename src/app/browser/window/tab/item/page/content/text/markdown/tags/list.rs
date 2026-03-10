use gtk::{
    TextBuffer, TextTag,
    prelude::{TextBufferExt, TextBufferExtManual},
};
use regex::Regex;

const REGEX_LIST: &str =
    r"(?m)^(?P<level>[ \t]*)\*[ \t]+(?:(?P<state>\[[ xX]\])[ \t]+)?(?P<text>.*)";

struct State(bool);

impl State {
    fn parse(value: Option<&str>) -> Option<Self> {
        if let Some(state) = value
            && (state.starts_with("[ ]") || state.starts_with("[x]"))
        {
            return Some(Self(state.starts_with("[x]")));
        }
        None
    }
    fn is_checked(&self) -> bool {
        self.0
    }
}

struct Item {
    pub level: usize,
    pub state: Option<State>,
    pub text: String,
}

impl Item {
    fn parse(level: &str, state: Option<&str>, text: String) -> Self {
        Self {
            level: level.chars().count(),
            state: State::parse(state),
            text,
        }
    }
}

/// Apply * list item `Tag` to given `TextBuffer`
pub fn render(buffer: &TextBuffer) {
    let state_tag = TextTag::builder().family("monospace").build();
    assert!(buffer.tag_table().add(&state_tag));

    let (start, end) = buffer.bounds();
    let full_content = buffer.text(&start, &end, true).to_string();

    let matches: Vec<_> = Regex::new(REGEX_LIST)
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

        let item = Item::parse(
            &cap["level"],
            cap.name("state").map(|m| m.as_str()),
            cap["text"].into(),
        );

        buffer.insert_with_tags(
            &mut start_iter,
            &format!("{}• ", " ".repeat(item.level)),
            &[],
        );
        if let Some(state) = item.state {
            buffer.insert_with_tags(
                &mut start_iter,
                if state.is_checked() { "[x] " } else { "[ ] " },
                &[&state_tag],
            );
        }
        buffer.insert_with_tags(&mut start_iter, &item.text, &[]);
    }
}

#[test]
fn test_regex() {
    fn item(cap: &Vec<regex::Captures<'_>>, n: usize) -> Item {
        let c = cap.get(n).unwrap();
        Item::parse(
            &c["level"],
            c.name("state").map(|m| m.as_str()),
            c["text"].into(),
        )
    }
    let cap: Vec<_> = Regex::new(REGEX_LIST)
        .unwrap()
        .captures_iter("Some\n* list item 1\n  * list item 1.1\n  * list item 1.2\n* list item 2\nand\n* list item 3\n  * [x] list item 3.1\n  * [ ] list item 3.2\n* list item 4\n")
        .collect();
    {
        let item = item(&cap, 0);
        assert_eq!(item.level, 0);
        assert!(item.state.is_none());
        assert_eq!(item.text, "list item 1");
    }
    {
        let item = item(&cap, 1);
        assert_eq!(item.level, 2);
        assert!(item.state.is_none());
        assert_eq!(item.text, "list item 1.1");
    }
    {
        let item = item(&cap, 2);
        assert_eq!(item.level, 2);
        assert!(item.state.is_none());
        assert_eq!(item.text, "list item 1.2");
    }
    {
        let item = item(&cap, 3);
        assert_eq!(item.level, 0);
        assert!(item.state.is_none());
        assert_eq!(item.text, "list item 2");
    }
    {
        let item = item(&cap, 4);
        assert_eq!(item.level, 0);
        assert!(item.state.is_none());
        assert_eq!(item.text, "list item 3");
    }
    {
        let item = item(&cap, 5);
        assert_eq!(item.level, 2);
        assert!(item.state.is_some_and(|this| this.is_checked()));
        assert_eq!(item.text, "list item 3.1");
    }
    {
        let item = item(&cap, 6);
        assert_eq!(item.level, 2);
        assert!(item.state.is_some_and(|this| !this.is_checked()));
        assert_eq!(item.text, "list item 3.2");
    }
    {
        let item = item(&cap, 7);
        assert_eq!(item.level, 0);
        assert!(item.state.is_none());
        assert_eq!(item.text, "list item 4");
    }
}
