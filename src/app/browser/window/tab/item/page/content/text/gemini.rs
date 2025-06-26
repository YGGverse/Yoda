mod ansi;
pub mod error;
mod gutter;
mod icon;
mod syntax;
mod tag;

pub use error::Error;
use gutter::Gutter;
use icon::Icon;
use syntax::Syntax;
use tag::Tag;

use super::{ItemAction, WindowAction};
use crate::app::browser::window::action::Position;
use gtk::{
    EventControllerMotion, GestureClick, TextBuffer, TextTag, TextView, TextWindowType,
    UriLauncher, Window, WrapMode,
    gdk::{BUTTON_MIDDLE, BUTTON_PRIMARY, RGBA},
    gio::Cancellable,
    glib::Uri,
    prelude::{TextBufferExt, TextBufferExtManual, TextTagExt, TextViewExt, WidgetExt},
};
use std::{cell::Cell, collections::HashMap, rc::Rc};

pub const NEW_LINE: &str = "\n";

pub struct Gemini {
    pub title: Option<String>,
    pub text_view: TextView,
}

impl Gemini {
    // Constructors

    /// Build new `Self`
    pub fn build(
        (window_action, item_action): (&Rc<WindowAction>, &Rc<ItemAction>),
        base: &Uri,
        gemtext: &str,
    ) -> Result<Self, Error> {
        // Init default values
        let mut title = None;

        // Init HashMap storage (for event controllers)
        let mut links: HashMap<TextTag, Uri> = HashMap::new();

        // Init hovered tag storage for `links`
        // * maybe less expensive than update entire HashMap by iter
        let hover: Rc<Cell<Option<TextTag>>> = Rc::new(Cell::new(None));

        // Init code features
        let mut code = None;

        // Init quote icon feature
        let mut is_line_after_quote = false;

        // Init colors
        // @TODO use accent colors in adw 1.6 / ubuntu 24.10+
        let link_color = (
            RGBA::new(0.208, 0.518, 0.894, 1.0),
            RGBA::new(0.208, 0.518, 0.894, 0.9),
        );

        // Init syntect highlight features
        let syntax = Syntax::new();

        // Init icons
        let icon = Icon::new();

        // Init tags
        let tag = Tag::new();

        // Init new text buffer
        let buffer = TextBuffer::new(Some(&tag.text_tag_table));

        // Init main widget
        let text_view = {
            const MARGIN: i32 = 8;
            TextView::builder()
                .bottom_margin(MARGIN)
                .buffer(&buffer)
                .cursor_visible(false)
                .editable(false)
                .left_margin(MARGIN)
                .right_margin(MARGIN)
                .top_margin(MARGIN)
                .vexpand(true)
                .wrap_mode(WrapMode::Word)
                .build()
        };

        // Init gutter widget (the tooltip on URL tags hover)
        let gutter = Gutter::build(&text_view);

        // Disable code format on at least one closing tag not found
        // gemini://bbs.geminispace.org/s/Gemini/26031
        let is_code_enabled = {
            use ggemtext::line::code::{self};
            let mut t: usize = 0;
            for l in gemtext.lines() {
                if l.starts_with(code::TAG) {
                    t += 1;
                }
            }
            t == 0 || t % 2 == 0
        };

        // Parse gemtext lines
        for line in gemtext.lines() {
            if is_code_enabled {
                use ggemtext::line::Code;
                match code {
                    None => {
                        // Open tag found
                        if let Some(c) = Code::begin_from(line) {
                            // Begin next lines collection into the code buffer
                            code = Some(c);

                            // Skip other actions for this line
                            continue;
                        }
                    }
                    Some(ref mut c) => {
                        match c.continue_from(line) {
                            Ok(()) => {
                                // Close tag found:
                                if c.is_completed {
                                    // Is alt provided
                                    let alt = match c.alt {
                                        Some(ref alt) => {
                                            // Insert alt value to the main buffer
                                            buffer.insert_with_tags(
                                                &mut buffer.end_iter(),
                                                alt.as_str(),
                                                &[&tag.title],
                                            );

                                            // Append new line after alt text
                                            buffer.insert(&mut buffer.end_iter(), NEW_LINE);

                                            // Return value as wanted also for syntax highlight detection
                                            Some(alt)
                                        }
                                        None => None,
                                    };

                                    // Begin code block construction
                                    // Try auto-detect code syntax for given `value` and `alt` @TODO optional
                                    match syntax.highlight(&c.value, alt) {
                                        Ok(highlight) => {
                                            for (syntax_tag, entity) in highlight {
                                                // Register new tag
                                                if !tag.text_tag_table.add(&syntax_tag) {
                                                    todo!()
                                                }
                                                // Append tag to buffer
                                                buffer.insert_with_tags(
                                                    &mut buffer.end_iter(),
                                                    &entity,
                                                    &[&syntax_tag],
                                                );
                                            }
                                        }
                                        Err(_) => {
                                            // Try ANSI/SGR format (terminal emulation) @TODO optional
                                            for (syntax_tag, entity) in ansi::format(&c.value) {
                                                // Register new tag
                                                if !tag.text_tag_table.add(&syntax_tag) {
                                                    todo!()
                                                }
                                                // Append tag to buffer
                                                buffer.insert_with_tags(
                                                    &mut buffer.end_iter(),
                                                    &entity,
                                                    &[&syntax_tag],
                                                );
                                            }
                                        } // @TODO handle
                                    }

                                    // Reset
                                    code = None;
                                }

                                // Skip other actions for this line
                                continue;
                            }
                            Err(_) => todo!(),
                        }
                    }
                }
            }

            // Is header
            {
                use ggemtext::line::{Header, header::Level};
                if let Some(header) = Header::parse(line) {
                    buffer.insert_with_tags(
                        &mut buffer.end_iter(),
                        &header.value,
                        &[match header.level {
                            Level::H1 => &tag.h1,
                            Level::H2 => &tag.h2,
                            Level::H3 => &tag.h3,
                        }],
                    );
                    buffer.insert(&mut buffer.end_iter(), NEW_LINE);

                    if title.is_none() {
                        title = Some(header.value.clone());
                    }
                    continue;
                }
            }

            // Is link
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

                    if !tag.text_tag_table.add(&a) {
                        panic!()
                    }

                    buffer.insert_with_tags(&mut buffer.end_iter(), &alt.join(" "), &[&a]);
                    buffer.insert(&mut buffer.end_iter(), NEW_LINE);

                    links.insert(a, uri);
                }
                continue;
            }

            // Is list

            if let Some(value) = ggemtext::line::list::Gemtext::as_value(line) {
                buffer.insert_with_tags(
                    &mut buffer.end_iter(),
                    &format!("• {value}"),
                    &[&tag.list],
                );
                buffer.insert(&mut buffer.end_iter(), NEW_LINE);
                continue;
            }

            // Is quote

            if let Some(quote) = ggemtext::line::quote::Gemtext::as_value(line) {
                // Show quote indicator if last line is not quote (to prevent duplicates)
                if !is_line_after_quote {
                    // Show only if the icons resolved for default `Display`
                    if let Some(ref icon) = icon {
                        buffer.insert_paintable(&mut buffer.end_iter(), &icon.quote);
                        buffer.insert(&mut buffer.end_iter(), NEW_LINE);
                    }
                }
                buffer.insert_with_tags(&mut buffer.end_iter(), quote, &[&tag.quote]);
                buffer.insert(&mut buffer.end_iter(), NEW_LINE);
                is_line_after_quote = true;
                continue;
            } else {
                is_line_after_quote = false;
            }

            // Nothing match custom tags above,
            // just append plain text covered in empty tag (to handle controller events properly)
            buffer.insert_with_tags(&mut buffer.end_iter(), line, &[&tag.plain]);
            buffer.insert(&mut buffer.end_iter(), NEW_LINE);
        }

        // Init additional controllers
        let primary_button_controller = GestureClick::builder().button(BUTTON_PRIMARY).build();
        let middle_button_controller = GestureClick::builder().button(BUTTON_MIDDLE).build();
        let motion_controller = EventControllerMotion::new();

        text_view.add_controller(primary_button_controller.clone());
        text_view.add_controller(middle_button_controller.clone());
        text_view.add_controller(motion_controller.clone());

        // Init shared reference container for HashTable collected
        let links = Rc::new(links);

        // Init events
        primary_button_controller.connect_released({
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
                                    |result| {
                                        if let Err(e) = result {
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

        middle_button_controller.connect_pressed({
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
                                    |result| {
                                        if let Err(e) = result {
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

        motion_controller.connect_motion({
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
                            gutter.set_uri(Some(uri));
                            // Toggle cursor
                            text_view.set_cursor_from_name(Some("pointer"));
                            // Redraw required to apply changes immediately
                            text_view.queue_draw();
                            return;
                        }
                    }
                }
                // Restore defaults
                gutter.set_uri(None);
                text_view.set_cursor_from_name(Some("text"));
                text_view.queue_draw();
            }
        }); // @TODO may be expensive for CPU, add timeout?

        // Result
        if is_code_enabled {
            Ok(Self { text_view, title })
        } else {
            Err(Error::Markup(
                "Invalid multiline markup! Gemtext format partially ignored.".to_string(),
                Self { text_view, title },
            ))
        }
    }
}
