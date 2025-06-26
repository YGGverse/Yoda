mod gutter;

use super::{ItemAction, WindowAction};
use crate::app::browser::window::action::Position;
use gtk::{
    EventControllerMotion, GestureClick, TextBuffer, TextTag, TextTagTable, TextView,
    TextWindowType, UriLauncher, Window, WrapMode,
    gdk::RGBA,
    gio::Cancellable,
    glib::Uri,
    prelude::{TextBufferExt, TextBufferExtManual, TextTagExt, TextViewExt, WidgetExt},
};
use gutter::Gutter;
use std::{cell::Cell, collections::HashMap, rc::Rc};

pub trait Nex {
    fn nex(actions: (&Rc<WindowAction>, &Rc<ItemAction>), base: &Uri, data: &str) -> Self;
}

impl Nex for TextView {
    fn nex(
        (window_action, item_action): (&Rc<WindowAction>, &Rc<ItemAction>),
        base: &Uri,
        data: &str,
    ) -> Self {
        pub const NEW_LINE: &str = "\n";

        // Init tags
        let tags = TextTagTable::new();

        // Define default tag once
        let plain_text_tag = TextTag::builder().wrap_mode(WrapMode::Word).build();
        tags.add(&plain_text_tag);

        // Init HashMap storage (for event controllers)
        let mut links: HashMap<TextTag, Uri> = HashMap::new();

        // Init hovered tag storage for `links`
        // * maybe less expensive than update entire HashMap by iter
        let hover: Rc<Cell<Option<TextTag>>> = Rc::new(Cell::new(None));

        // Init colors
        // @TODO use accent colors in adw 1.6 / ubuntu 24.10+
        let link_color = (
            RGBA::new(0.208, 0.518, 0.894, 1.0),
            RGBA::new(0.208, 0.518, 0.894, 0.9),
        );

        // Init new text buffer
        let buffer = TextBuffer::new(Some(&tags));

        // Collect links
        for line in data.lines() {
            // just borrow ggemtext parser as compatible API
            if let Some(link) = ggemtext::line::Link::parse(line) {
                if let Some(uri) = link.uri(Some(base)) {
                    let mut alt = Vec::new();

                    if uri.scheme() != base.scheme() {
                        alt.push("⇖".to_string());
                    }

                    alt.push(match link.alt {
                        Some(alt) => alt,
                        None => uri.to_string(),
                    });

                    let a = TextTag::builder()
                        .foreground_rgba(&link_color.0)
                        // .foreground_rgba(&adw::StyleManager::default().accent_color_rgba()) @TODO adw 1.6 / ubuntu 24.10+
                        .sentence(true)
                        .wrap_mode(WrapMode::Word)
                        .build();

                    if !tags.add(&a) {
                        panic!()
                    }

                    buffer.insert_with_tags(&mut buffer.end_iter(), &alt.join(" "), &[&a]);
                    buffer.insert(&mut buffer.end_iter(), NEW_LINE);

                    links.insert(a, uri);

                    continue;
                }
            }
            // Nothing match custom tags above,
            // just append plain text covered in empty tag (to handle controller events properly)
            buffer.insert_with_tags(&mut buffer.end_iter(), line, &[&plain_text_tag]);
            buffer.insert(&mut buffer.end_iter(), NEW_LINE);
        }

        // Init main widget

        let text_view = {
            const MARGIN: i32 = 8;
            TextView::builder()
                .bottom_margin(MARGIN)
                .buffer(&buffer)
                .cursor_visible(false)
                .editable(false)
                .left_margin(MARGIN)
                .monospace(true)
                .right_margin(MARGIN)
                .top_margin(MARGIN)
                .vexpand(true)
                .wrap_mode(gtk::WrapMode::Word)
                .build()
        };

        // Init additional controllers
        text_view.add_controller({
            let c = GestureClick::builder()
                .button(gtk::gdk::BUTTON_PRIMARY)
                .build();
            c.connect_released({
                let item_action = item_action.clone();
                let links = links.clone();
                let text_view = text_view.clone();
                move |_, _, window_x, window_y| {
                    // Detect tag match current coords hovered
                    let (buffer_x, buffer_y) = text_view.window_to_buffer_coords(
                        TextWindowType::Widget,
                        window_x as i32,
                        window_y as i32,
                    );

                    if let Some(iter) = text_view.iter_at_location(buffer_x, buffer_y) {
                        for tag in iter.tags() {
                            // Tag is link
                            if let Some(uri) = links.get(&tag) {
                                // Select link handler by scheme
                                return match uri.scheme().as_str() {
                                    "gemini" | "titan" | "nex" | "file" => {
                                        item_action.load.activate(Some(&uri.to_str()), true, false)
                                    }
                                    // Scheme not supported, delegate
                                    _ => UriLauncher::new(&uri.to_str()).launch(
                                        Window::NONE,
                                        Cancellable::NONE,
                                        |r| {
                                            if let Err(e) = r {
                                                println!("{e}")
                                            }
                                        },
                                    ),
                                }; // @TODO common handler?
                            }
                        }
                    }
                }
            });
            c
        });

        text_view.add_controller({
            let c = GestureClick::builder()
                .button(gtk::gdk::BUTTON_MIDDLE)
                .build();
            c.connect_pressed({
                let links = links.clone();
                let text_view = text_view.clone();
                let window_action = window_action.clone();
                move |_, _, window_x, window_y| {
                    // Detect tag match current coords hovered
                    let (buffer_x, buffer_y) = text_view.window_to_buffer_coords(
                        TextWindowType::Widget,
                        window_x as i32,
                        window_y as i32,
                    );
                    if let Some(iter) = text_view.iter_at_location(buffer_x, buffer_y) {
                        for tag in iter.tags() {
                            // Tag is link
                            if let Some(uri) = links.get(&tag) {
                                // Select link handler by scheme
                                return match uri.scheme().as_str() {
                                    "gemini" | "titan" | "nex" | "file" => {
                                        // Open new page in browser
                                        window_action.append.activate_stateful_once(
                                            Position::After,
                                            Some(uri.to_string()),
                                            false,
                                            false,
                                            true,
                                            true,
                                        );
                                    }
                                    // Scheme not supported, delegate
                                    _ => UriLauncher::new(&uri.to_str()).launch(
                                        Window::NONE,
                                        Cancellable::NONE,
                                        |r| {
                                            if let Err(e) = r {
                                                println!("{e}")
                                            }
                                        },
                                    ),
                                }; // @TODO common handler?
                            }
                        }
                    }
                }
            }); // for a note: this action sensitive to focus out
            c
        });

        text_view.add_controller({
            // Init gutter widget (the tooltip on URL tags hover)
            let g = Gutter::build(&text_view);
            let c = EventControllerMotion::new();
            c.connect_motion({
                let text_view = text_view.clone();
                let links = links.clone();
                let hover = hover.clone();
                move |_, window_x, window_y| {
                    // Detect tag match current coords hovered
                    let (buffer_x, buffer_y) = text_view.window_to_buffer_coords(
                        TextWindowType::Widget,
                        window_x as i32,
                        window_y as i32,
                    );
                    // Reset link colors to default
                    if let Some(tag) = hover.replace(None) {
                        tag.set_foreground_rgba(Some(&link_color.0));
                    }
                    // Apply hover effect
                    if let Some(iter) = text_view.iter_at_location(buffer_x, buffer_y) {
                        for tag in iter.tags() {
                            // Tag is link
                            if let Some(uri) = links.get(&tag) {
                                // Toggle color
                                tag.set_foreground_rgba(Some(&link_color.1));
                                // Keep hovered tag in memory
                                hover.replace(Some(tag.clone()));
                                // Show tooltip
                                g.set_uri(Some(uri));
                                // Toggle cursor
                                text_view.set_cursor_from_name(Some("pointer"));
                                // Redraw required to apply changes immediately
                                text_view.queue_draw();
                                return;
                            }
                        }
                    }
                    // Restore defaults
                    g.set_uri(None);
                    text_view.set_cursor_from_name(Some("text"));
                    text_view.queue_draw();
                }
            }); // @TODO may be expensive for CPU, add timeout?
            c
        });

        text_view
    }
}
