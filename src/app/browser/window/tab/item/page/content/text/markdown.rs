mod ansi;
pub mod error;
mod gutter;
mod icon;
mod reference;
mod syntax;
mod tag;

use super::{ItemAction, WindowAction};
use crate::app::browser::window::action::Position;
pub use error::Error;
use gtk::{
    EventControllerMotion, GestureClick, TextBuffer, TextTag, TextView, TextWindowType,
    UriLauncher, Window, WrapMode,
    gdk::{BUTTON_MIDDLE, BUTTON_PRIMARY, BUTTON_SECONDARY, RGBA},
    gio::{Cancellable, SimpleAction, SimpleActionGroup},
    glib::{Uri, uuid_string_random},
    prelude::{PopoverExt, TextBufferExt, TextBufferExtManual, TextTagExt, TextViewExt, WidgetExt},
};
use gutter::Gutter;
use icon::Icon;
use sourceview::prelude::{ActionExt, ActionMapExt, DisplayExt, ToVariant};
use std::{cell::Cell, collections::HashMap, rc::Rc};
use syntax::Syntax;
use tag::Tag;

pub const NEW_LINE: &str = "\n";

pub struct Markdown {
    pub title: Option<String>,
    pub text_view: TextView,
}

impl Markdown {
    // Constructors

    /// Build new `Self`
    pub fn build(
        (window_action, item_action): (&Rc<WindowAction>, &Rc<ItemAction>),
        base: &Uri,
        markdown: &str,
    ) -> Result<Self, Error> {
        // Init default values
        let mut title = None;

        // Init HashMap storage (for event controllers)
        let mut links: HashMap<TextTag, Uri> = HashMap::new();

        // Init hovered tag storage for `links`
        // * maybe less expensive than update entire HashMap by iter
        let hover: Rc<Cell<Option<TextTag>>> = Rc::new(Cell::new(None));

        // Init code features
        //let mut code = None;

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
        buffer.set_text(markdown);

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
            for l in markdown.lines() {
                if l.starts_with(code::TAG) {
                    t += 1;
                }
            }
            t == 0 || t.is_multiple_of(2)
        };

        // Parse in-line markdown tags
        // * keep order!

        reference::image_link(&buffer, &tag, base, &link_color.0, &mut links);
        reference::image(&buffer, &tag, base, &link_color.0, &mut links);
        reference::link(&buffer, &tag, base, &link_color.0, &mut links);

        // Parse single-line markdown tags
        /*'l: for line in markdown.lines() {
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

            // Is 1-6 level header
            for level in 1..=6 {
                if let Some(t) = header(
                    &buffer,
                    match level {
                        1 => &tag.h1,
                        2 => &tag.h2,
                        3 => &tag.h3,
                        4 => &tag.h4,
                        5 => &tag.h5,
                        6 => &tag.h6,
                        _ => unreachable!(),
                    },
                    line,
                    &H.repeat(level),
                ) {
                    // Update document title by tag, if not set before
                    if title.is_none() {
                        title = Some(t);
                    }
                    continue 'l;
                }
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
        }*/

        // Context menu
        let action_link_tab =
            SimpleAction::new_stateful(&uuid_string_random(), None, &String::new().to_variant());
        action_link_tab.connect_activate({
            let window_action = window_action.clone();
            move |this, _| {
                open_link_in_new_tab(
                    &this.state().unwrap().get::<String>().unwrap(),
                    &window_action,
                )
            }
        });
        let action_link_copy =
            SimpleAction::new_stateful(&uuid_string_random(), None, &String::new().to_variant());
        action_link_copy.connect_activate(|this, _| {
            gtk::gdk::Display::default()
                .unwrap()
                .clipboard()
                .set_text(&this.state().unwrap().get::<String>().unwrap())
        });
        let action_link_download =
            SimpleAction::new_stateful(&uuid_string_random(), None, &String::new().to_variant());
        action_link_download.connect_activate({
            let window_action = window_action.clone();
            move |this, _| {
                open_link_in_new_tab(
                    &link_prefix(
                        this.state().unwrap().get::<String>().unwrap(),
                        LINK_PREFIX_DOWNLOAD,
                    ),
                    &window_action,
                )
            }
        });
        let action_link_source =
            SimpleAction::new_stateful(&uuid_string_random(), None, &String::new().to_variant());
        action_link_source.connect_activate({
            let window_action = window_action.clone();
            move |this, _| {
                open_link_in_new_tab(
                    &link_prefix(
                        this.state().unwrap().get::<String>().unwrap(),
                        LINK_PREFIX_SOURCE,
                    ),
                    &window_action,
                )
            }
        });
        let link_context_group_id = uuid_string_random();
        text_view.insert_action_group(
            &link_context_group_id,
            Some(&{
                let g = SimpleActionGroup::new();
                g.add_action(&action_link_tab);
                g.add_action(&action_link_copy);
                g.add_action(&action_link_download);
                g.add_action(&action_link_source);
                g
            }),
        );
        let link_context = gtk::PopoverMenu::from_model(Some(&{
            let m = gtk::gio::Menu::new();
            m.append(
                Some("Open Link in New Tab"),
                Some(&format!(
                    "{link_context_group_id}.{}",
                    action_link_tab.name()
                )),
            );
            m.append(
                Some("Copy Link"),
                Some(&format!(
                    "{link_context_group_id}.{}",
                    action_link_copy.name()
                )),
            );
            m.append(
                Some("Download Link"),
                Some(&format!(
                    "{link_context_group_id}.{}",
                    action_link_download.name()
                )),
            );
            m.append(
                Some("View Link as Source"),
                Some(&format!(
                    "{link_context_group_id}.{}",
                    action_link_source.name()
                )),
            );
            m
        }));
        link_context.set_parent(&text_view);

        // Init additional controllers
        let middle_button_controller = GestureClick::builder().button(BUTTON_MIDDLE).build();
        let primary_button_controller = GestureClick::builder().button(BUTTON_PRIMARY).build();
        let secondary_button_controller = GestureClick::builder()
            .button(BUTTON_SECONDARY)
            .propagation_phase(gtk::PropagationPhase::Capture)
            .build();
        let motion_controller = EventControllerMotion::new();

        text_view.add_controller(middle_button_controller.clone());
        text_view.add_controller(motion_controller.clone());
        text_view.add_controller(primary_button_controller.clone());
        text_view.add_controller(secondary_button_controller.clone());

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
                            return open_link_in_current_tab(&uri.to_string(), &item_action);
                        }
                    }
                }
            }
        });

        secondary_button_controller.connect_pressed({
            let links = links.clone();
            let text_view = text_view.clone();
            let link_context = link_context.clone();
            move |_, _, window_x, window_y| {
                let x = window_x as i32;
                let y = window_y as i32;
                // Detect tag match current coords hovered
                let (buffer_x, buffer_y) =
                    text_view.window_to_buffer_coords(TextWindowType::Widget, x, y);
                if let Some(iter) = text_view.iter_at_location(buffer_x, buffer_y) {
                    for tag in iter.tags() {
                        // Tag is link
                        if let Some(uri) = links.get(&tag) {
                            let request_str = uri.to_str();
                            let request_var = request_str.to_variant();

                            action_link_tab.set_state(&request_var);
                            action_link_copy.set_state(&request_var);

                            action_link_download.set_state(&request_var);
                            action_link_download.set_enabled(is_prefixable_link(&request_str));

                            action_link_source.set_state(&request_var);
                            action_link_source.set_enabled(is_prefixable_link(&request_str));

                            link_context
                                .set_pointing_to(Some(&gtk::gdk::Rectangle::new(x, y, 1, 1)));
                            link_context.popup();
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
                            return open_link_in_new_tab(&uri.to_string(), &window_action);
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
                "Invalid multiline markup! Markdown format partially ignored.".to_string(),
                Self { text_view, title },
            ))
        }
    }
}

fn is_internal_link(request: &str) -> bool {
    // schemes
    request.starts_with("gemini://")
        || request.starts_with("titan://")
        || request.starts_with("nex://")
        || request.starts_with("file://")
        // prefix
        || request.starts_with("download:")
        || request.starts_with("source:")
}

fn is_prefixable_link(request: &str) -> bool {
    request.starts_with("gemini://")
        || request.starts_with("nex://")
        || request.starts_with("file://")
}

fn open_link_in_external_app(request: &str) {
    UriLauncher::new(request).launch(Window::NONE, Cancellable::NONE, |r| {
        if let Err(e) = r {
            println!("{e}") // @TODO use warn macro
        }
    })
}

fn open_link_in_current_tab(request: &str, item_action: &ItemAction) {
    if is_internal_link(request) {
        item_action.load.activate(Some(request), true, false)
    } else {
        open_link_in_external_app(request)
    }
}

fn open_link_in_new_tab(request: &str, window_action: &WindowAction) {
    if is_internal_link(request) {
        window_action.append.activate_stateful_once(
            Position::After,
            Some(request.into()),
            false,
            false,
            true,
            true,
        );
    } else {
        open_link_in_external_app(request)
    }
}

fn link_prefix(request: String, prefix: &str) -> String {
    format!("{prefix}{}", request.trim_start_matches(prefix))
}

/// Header tag
fn header(buffer: &TextBuffer, tag: &TextTag, line: &str, pattern: &str) -> Option<String> {
    if let Some(h) = line.trim_start().strip_prefix(pattern)
        && !h.starts_with(pattern)
    {
        let header = h.trim();
        buffer.insert_with_tags(&mut buffer.end_iter(), header, &[tag]);
        buffer.insert(&mut buffer.end_iter(), NEW_LINE);
        Some(header.into())
    } else {
        None
    }
}

const LINK_PREFIX_DOWNLOAD: &str = "download:";
const LINK_PREFIX_SOURCE: &str = "source:";

const H: &str = "#";
