mod ansi;
mod syntax;

use gtk::{
    Align, Box, Button, Label, Orientation, PolicyType, ScrolledWindow, Separator, TextBuffer,
    TextSearchFlags, TextTag, TextTagTable, TextView, WrapMode,
    gdk::Display,
    glib::{ControlFlow, GString, idle_add_local, uuid_string_random},
    prelude::{
        BoxExt, ButtonExt, DisplayExt, TextBufferExt, TextBufferExtManual, TextViewExt, WidgetExt,
    },
};
use regex::Regex;
use std::{cell::Cell, collections::HashMap, rc::Rc};
use syntax::Syntax;

const REGEX_CODE: &str = r"(?s)```[ \t]*(?P<alt>.*?)\n(?P<data>.*?)```";

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

    /// Apply code `Tag` to given `TextView` using `Self.index`
    pub fn render(&mut self, text_view: &TextView) {
        let buffer = text_view.buffer();
        let syntax = Syntax::new();
        let copied = Rc::new(Cell::new(None));

        assert!(buffer.tag_table().add(&self.alt));

        for (k, v) in self.index.iter() {
            while let Some((mut m_start, mut m_end)) =
                buffer
                    .start_iter()
                    .forward_search(k, TextSearchFlags::VISIBLE_ONLY, None)
            {
                buffer.delete(&mut m_start, &mut m_end);
                text_view.add_child_at_anchor(
                    &{
                        const MARGIN: i32 = 16;
                        let widget = Box::builder()
                            .css_classes(["card"])
                            .halign(Align::Fill)
                            .hexpand(true)
                            .margin_bottom(MARGIN / 2)
                            .orientation(Orientation::Vertical)
                            .build();
                        widget.append(&{
                            let header = Box::builder()
                                .halign(Align::Fill)
                                .hexpand(true)
                                .orientation(Orientation::Horizontal)
                                .build();
                            if let Some(ref alt) = v.alt {
                                header.append(
                                    &Label::builder()
                                        .halign(Align::Start)
                                        .hexpand(true)
                                        .label(alt)
                                        .margin_bottom(MARGIN)
                                        .margin_end(MARGIN)
                                        .margin_start(MARGIN)
                                        .margin_top(MARGIN)
                                        .selectable(true)
                                        .build(),
                                );
                            }
                            header.append(&{
                                const TOGGLE_BUTTON_CLASS: &str = "dimmed";
                                const TOGGLE_BUTTON_TOOLTIP: (&str, &str) = ("Copy", "Copied");
                                let copy = Button::builder()
                                    .css_classes(["circular", "flat", TOGGLE_BUTTON_CLASS])
                                    .halign(Align::End)
                                    .icon_name("edit-copy-symbolic")
                                    .margin_bottom(MARGIN / 2)
                                    .margin_end(MARGIN / 2)
                                    .margin_start(MARGIN / 2)
                                    .margin_top(MARGIN / 2)
                                    .tooltip_text(TOGGLE_BUTTON_TOOLTIP.0)
                                    .valign(Align::Center)
                                    .build();
                                copy.set_cursor_from_name(Some("pointer"));
                                copy.connect_clicked({
                                    let source = v.data.clone();
                                    let copied = copied.clone();
                                    move |this| {
                                        if let Some(prev) = copied.replace(Some(this.clone())) {
                                            prev.set_tooltip_text(Some(TOGGLE_BUTTON_TOOLTIP.0));
                                            prev.add_css_class(TOGGLE_BUTTON_CLASS)
                                        }
                                        this.set_tooltip_text(Some(TOGGLE_BUTTON_TOOLTIP.1));
                                        this.remove_css_class(TOGGLE_BUTTON_CLASS);

                                        Display::default().unwrap().clipboard().set_text(&source)
                                    }
                                });
                                copy
                            });
                            header
                        });
                        widget.append(
                            &Separator::builder()
                                .orientation(Orientation::Horizontal)
                                .build(),
                        );
                        widget.append(&{
                            ScrolledWindow::builder()
                                .child(
                                    &TextView::builder()
                                        .buffer(&{
                                            let b = TextBuffer::new(Some(&TextTagTable::new()));
                                            let mut start = b.start_iter();
                                            match syntax.highlight(&v.data, v.alt.as_ref()) {
                                                Ok(highlight) => {
                                                    for (syntax_tag, entity) in highlight {
                                                        assert!(b.tag_table().add(&syntax_tag));
                                                        b.insert_with_tags(
                                                            &mut start,
                                                            &entity,
                                                            &[&syntax_tag],
                                                        )
                                                    }
                                                }
                                                Err(_) => {
                                                    // Try ANSI/SGR format (terminal emulation) @TODO optional
                                                    for (syntax_tag, entity) in
                                                        ansi::format(&v.data)
                                                    {
                                                        assert!(b.tag_table().add(&syntax_tag));
                                                        b.insert_with_tags(
                                                            &mut start,
                                                            &entity,
                                                            &[&syntax_tag],
                                                        )
                                                    }
                                                }
                                            }
                                            b
                                        })
                                        .css_classes(["code-block"])
                                        .cursor_visible(false)
                                        .editable(false)
                                        .wrap_mode(WrapMode::None)
                                        .build(),
                                )
                                .margin_end(MARGIN)
                                .margin_start(MARGIN)
                                .margin_top(MARGIN)
                                .vscrollbar_policy(PolicyType::Never)
                                .hscrollbar_policy(PolicyType::Automatic)
                                .propagate_natural_height(true)
                                .build()
                        });
                        idle_add_local({
                            let widget = widget.clone();
                            let text_view = text_view.clone();
                            move || {
                                widget.set_width_request(text_view.width() - 22);
                                ControlFlow::Break
                            }
                        });
                        widget
                    },
                    &buffer.create_child_anchor(&mut m_end),
                );
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
