use gtk::{
    Orientation, Separator, TextView,
    glib::{ControlFlow, idle_add_local},
    prelude::*,
};
use regex::Regex;

const REGEX_HR: &str = r"(?m)^(?P<hr>\\?[-]{3,})$";

/// Apply --- `Tag` to given `TextBuffer`
pub fn render(text_view: &TextView) {
    let separator = Separator::builder()
        .orientation(Orientation::Horizontal)
        .build();
    idle_add_local({
        let text_view = text_view.clone();
        let separator = separator.clone();
        move || {
            separator.set_width_request(text_view.width() - 18);
            ControlFlow::Break
        }
    });

    let buffer = text_view.buffer();

    let (start, end) = buffer.bounds();
    let full_content = buffer.text(&start, &end, true).to_string();

    let matches: Vec<_> = Regex::new(REGEX_HR)
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

        buffer.delete(&mut start_iter, &mut end_iter);
        text_view.add_child_at_anchor(&separator, &buffer.create_child_anchor(&mut end_iter));
    }
}

pub fn strip_tags(value: &str) -> String {
    let mut result = String::from(value);
    for cap in Regex::new(REGEX_HR).unwrap().captures_iter(value) {
        if let Some(m) = cap.get(0) {
            result = result.replace(m.as_str(), &cap["hr"]);
        }
    }
    result
}

#[test]
fn test_strip_tags() {
    const VALUE: &str = "Some line\n---\nSome another-line with ![img](https://link.com)";
    let mut result = String::from(VALUE);
    for cap in Regex::new(REGEX_HR).unwrap().captures_iter(VALUE) {
        if let Some(m) = cap.get(0) {
            result = result.replace(m.as_str(), "");
        }
    }
    assert_eq!(
        result,
        "Some line\n\nSome another-line with ![img](https://link.com)"
    )
}

#[test]
fn test_regex() {
    let cap: Vec<_> = Regex::new(REGEX_HR)
        .unwrap()
        .captures_iter("Some line\n---\nSome another-line with ![img](https://link.com)")
        .collect();

    assert_eq!(&cap.first().unwrap()["hr"], "---");
}
