mod ansi;
mod syntax;

use gtk::{
    Align, Box, Button, Label, Orientation, PolicyType, ScrolledWindow, Separator, TextBuffer,
    TextTagTable, TextView, WrapMode,
    gdk::Display,
    glib::{ControlFlow, idle_add_local},
    prelude::{BoxExt, ButtonExt, DisplayExt, TextBufferExt, TextBufferExtManual, WidgetExt},
};
use std::{cell::Cell, rc::Rc};
use syntax::Syntax;

pub struct Code {
    pub widget: Box,
}

impl Code {
    pub fn init(parent: &TextView, source: &str, alt: Option<&String>) -> Self {
        let syntax = Syntax::new();
        let copied = Rc::new(Cell::new(None));

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
            if let Some(label) = alt {
                header.append(
                    &Label::builder()
                        .halign(Align::Start)
                        .hexpand(true)
                        .label(label)
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
                    let source = String::from(source);
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
                            match syntax.highlight(source, alt) {
                                Ok(highlight) => {
                                    for (syntax_tag, entity) in highlight {
                                        assert!(b.tag_table().add(&syntax_tag));
                                        b.insert_with_tags(&mut start, &entity, &[&syntax_tag])
                                    }
                                }
                                Err(_) => {
                                    // Try ANSI/SGR format (terminal emulation) @TODO optional
                                    for (syntax_tag, entity) in ansi::format(source) {
                                        assert!(b.tag_table().add(&syntax_tag));
                                        b.insert_with_tags(&mut start, &entity, &[&syntax_tag])
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
                .margin_bottom(MARGIN)
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
            let parent = parent.clone();
            move || {
                widget.set_width_request(parent.width() - 22);
                ControlFlow::Break
            }
        });
        Self { widget }
    }
}

const MARGIN: i32 = 16;
