mod ansi;
pub mod error;
mod icon;
mod syntax;
mod tag;

pub use error::Error;
use icon::Icon;
use syntax::Syntax;
use tag::Tag;

use super::{ItemAction, WindowAction};
use crate::app::browser::window::action::Position;
use ggemtext::line::{
    code::{Inline, Multiline},
    header::{Header, Level},
    link::Link,
    list::List,
    quote::Quote,
};
use gtk::{
    gdk::{BUTTON_MIDDLE, BUTTON_PRIMARY, RGBA},
    gio::Cancellable,
    glib::{TimeZone, Uri},
    prelude::{TextBufferExt, TextBufferExtManual, TextTagExt, TextViewExt, WidgetExt},
    EventControllerMotion, GestureClick, TextBuffer, TextTag, TextView, TextWindowType,
    UriLauncher, Window, WrapMode,
};
use std::{cell::Cell, collections::HashMap, rc::Rc};

pub const DATE_FORMAT: &str = "%Y-%m-%d";
pub const EXTERNAL_LINK_INDICATOR: &str = "⇖";
pub const LIST_ITEM: &str = "•";
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

        // Init multiline code builder features
        let mut multiline = None;

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

        // Parse gemtext lines
        for line in gemtext.lines() {
            // Is inline code
            if let Some(code) = Inline::from(line) {
                // Try auto-detect code syntax for given `value` @TODO optional
                match syntax.highlight(&code.value, None) {
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
                        for (ansi_tag, entity) in ansi::format(&code.value) {
                            // Register new tag
                            if !tag.text_tag_table.add(&ansi_tag) {
                                todo!()
                            }
                            // Append tag to buffer
                            buffer.insert_with_tags(&mut buffer.end_iter(), &entity, &[&ansi_tag]);
                        }
                    } // @TODO handle
                }

                // Append new line
                buffer.insert(&mut buffer.end_iter(), NEW_LINE);

                // Skip other actions for this line
                continue;
            }

            // Is multiline code
            match multiline {
                None => {
                    // Open tag found
                    if let Some(code) = Multiline::begin_from(line) {
                        // Begin next lines collection into the code buffer
                        multiline = Some(code);

                        // Skip other actions for this line
                        continue;
                    }
                }
                Some(ref mut this) => {
                    match Multiline::continue_from(this, line) {
                        Ok(()) => {
                            // Close tag found:
                            if this.completed {
                                // Is alt provided
                                let alt = match this.alt {
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
                                match syntax.highlight(&this.value, alt) {
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
                                        for (syntax_tag, entity) in ansi::format(&this.value) {
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
                                multiline = None;
                            }

                            // Skip other actions for this line
                            continue;
                        }
                        Err(e) => return Err(Error::Gemtext(e.to_string())),
                    }
                }
            };

            // Is header
            if let Some(header) = Header::from(line) {
                // Append value to buffer
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

                // Update reader title using first gemtext header match
                if title.is_none() {
                    title = Some(header.value.clone());
                }

                // Skip other actions for this line
                continue;
            }

            // Is link
            if let Some(link) = Link::from(line, Some(base), Some(&TimeZone::local())) {
                // Create vector for alt values
                let mut alt = Vec::new();

                // Append external indicator on exist
                if let Some(is_external) = link.is_external {
                    if is_external {
                        alt.push(EXTERNAL_LINK_INDICATOR.to_string());
                    }
                }

                // Append date on exist
                if let Some(timestamp) = link.timestamp {
                    // https://docs.gtk.org/glib/method.DateTime.format.html
                    if let Ok(value) = timestamp.format(DATE_FORMAT) {
                        alt.push(value.to_string())
                    }
                }

                // Append alt value on exist or use URL
                alt.push(match link.alt {
                    Some(alt) => alt.to_string(),
                    None => link.uri.to_string(),
                });

                // Create new tag for new link
                let a = TextTag::builder()
                    .foreground_rgba(&link_color.0)
                    // .foreground_rgba(&adw::StyleManager::default().accent_color_rgba()) @TODO adw 1.6 / ubuntu 24.10+
                    .sentence(true)
                    .wrap_mode(WrapMode::Word)
                    .build();

                // Register new tag
                if !tag.text_tag_table.add(&a) {
                    todo!()
                }

                // Append alt vector values to buffer
                buffer.insert_with_tags(&mut buffer.end_iter(), &alt.join(" "), &[&a]);
                buffer.insert(&mut buffer.end_iter(), NEW_LINE);

                // Append tag to HashMap storage
                links.insert(a, link.uri.clone());

                // Skip other actions for this line
                continue;
            }

            // Is list
            if let Some(list) = List::from(line) {
                // Append value to buffer
                buffer.insert_with_tags(
                    &mut buffer.end_iter(),
                    format!("{LIST_ITEM} {}", list.value).as_str(),
                    &[&tag.list],
                );
                buffer.insert(&mut buffer.end_iter(), NEW_LINE);

                // Skip other actions for this line
                continue;
            }

            // Is quote
            if let Some(quote) = Quote::from(line) {
                // Show quote indicator if last line is not quote (to prevent duplicates)
                if !is_line_after_quote {
                    // Show only if the icons resolved for default `Display`
                    if let Some(ref icon) = icon {
                        buffer.insert_paintable(&mut buffer.end_iter(), &icon.quote);
                        buffer.insert(&mut buffer.end_iter(), NEW_LINE);
                    }
                }
                is_line_after_quote = true;

                // Append value to buffer
                buffer.insert_with_tags(&mut buffer.end_iter(), &quote.value, &[&tag.quote]);
                buffer.insert(&mut buffer.end_iter(), NEW_LINE);

                // Skip other actions for this line
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
                    TextWindowType::Text,
                    window_x as i32,
                    window_y as i32,
                );

                if let Some(iter) = text_view.iter_at_location(buffer_x, buffer_y) {
                    for tag in iter.tags() {
                        // Tag is link
                        if let Some(uri) = links.get(&tag) {
                            // Select link handler by scheme
                            return match uri.scheme().as_str() {
                                "gemini" | "titan" => {
                                    // Open new page in browser
                                    item_action.load.activate(Some(&uri.to_str()), true);
                                }
                                // Scheme not supported, delegate
                                _ => UriLauncher::new(&uri.to_str()).launch(
                                    Window::NONE,
                                    Cancellable::NONE,
                                    |result| {
                                        if let Err(error) = result {
                                            println!("{error}")
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
                    TextWindowType::Text,
                    window_x as i32,
                    window_y as i32,
                );
                if let Some(iter) = text_view.iter_at_location(buffer_x, buffer_y) {
                    for tag in iter.tags() {
                        // Tag is link
                        if let Some(uri) = links.get(&tag) {
                            // Select link handler by scheme
                            return match uri.scheme().as_str() {
                                "gemini" | "titan" => {
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
                    TextWindowType::Text,
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

                            // Toggle cursor
                            text_view.set_cursor_from_name(Some("pointer"));

                            // Show tooltip | @TODO set_gutter option?
                            text_view.set_tooltip_text(Some(&uri.to_string()));

                            // Redraw required to apply changes immediately
                            text_view.queue_draw();

                            return;
                        }
                    }
                }

                // Restore defaults
                text_view.set_cursor_from_name(Some("text"));
                text_view.set_tooltip_text(None);
                text_view.queue_draw();
            }
        }); // @TODO may be expensive for CPU, add timeout?

        // Result
        Ok(Self { text_view, title })
    }
}
