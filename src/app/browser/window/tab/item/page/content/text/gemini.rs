mod ansi;
pub mod error;
mod gutter;
mod icon;
mod separator;
mod syntax;
mod tag;

use super::{ItemAction, WindowAction};
use crate::{app::browser::window::action::Position, profile::Profile};
pub use error::Error;
use gtk::{
    EventControllerMotion, GestureClick, TextBuffer, TextTag, TextView, TextWindowType,
    UriLauncher, Window, WrapMode,
    gdk::{BUTTON_MIDDLE, BUTTON_PRIMARY, BUTTON_SECONDARY, Display, RGBA},
    gio::{Cancellable, Menu, SimpleAction, SimpleActionGroup},
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

pub struct Gemini {
    pub title: Option<String>,
    pub text_view: TextView,
}

impl Gemini {
    // Constructors

    /// Build new `Self`
    pub fn build(
        (window_action, item_action): (&Rc<WindowAction>, &Rc<ItemAction>),
        profile: &Rc<Profile>,
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
            t == 0 || t.is_multiple_of(2)
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

                                    text_view.add_child_at_anchor(
                                        &separator::horizontal(&text_view),
                                        &buffer.create_child_anchor(&mut buffer.end_iter()),
                                    );

                                    // Begin code block construction
                                    // Try auto-detect code syntax for given `value` and `alt` @TODO optional
                                    match syntax.highlight(&c.value, alt) {
                                        Ok(highlight) => {
                                            for (syntax_tag, entity) in highlight {
                                                assert!(tag.text_tag_table.add(&syntax_tag));
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
                                                assert!(tag.text_tag_table.add(&syntax_tag));
                                                buffer.insert_with_tags(
                                                    &mut buffer.end_iter(),
                                                    &entity,
                                                    &[&syntax_tag],
                                                );
                                            }
                                        } // @TODO handle
                                    }

                                    text_view.add_child_at_anchor(
                                        &separator::horizontal(&text_view),
                                        &buffer.create_child_anchor(&mut buffer.end_iter()),
                                    );

                                    // Reset
                                    code = None;
                                }

                                // Skip other actions for this line
                                continue;
                            }
                            Err(_) => panic!(),
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
                    let mut alt = Vec::with_capacity(2);

                    if uri.scheme() != base.scheme() {
                        alt.push(LINK_EXTERNAL_INDICATOR.to_string());
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

                    assert!(tag.text_tag_table.add(&a));

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
        let action_link_copy_url =
            SimpleAction::new_stateful(&uuid_string_random(), None, &String::new().to_variant());
        action_link_copy_url.connect_activate(|this, _| {
            Display::default()
                .unwrap()
                .clipboard()
                .set_text(&this.state().unwrap().get::<String>().unwrap())
        });
        let action_link_copy_text =
            SimpleAction::new_stateful(&uuid_string_random(), None, &String::new().to_variant());
        action_link_copy_text.connect_activate(|this, _| {
            Display::default()
                .unwrap()
                .clipboard()
                .set_text(&this.state().unwrap().get::<String>().unwrap())
        });
        let action_link_copy_text_selected =
            SimpleAction::new_stateful(&uuid_string_random(), None, &String::new().to_variant());
        action_link_copy_text_selected.connect_activate(|this, _| {
            Display::default()
                .unwrap()
                .clipboard()
                .set_text(&this.state().unwrap().get::<String>().unwrap())
        });
        let action_link_bookmark =
            SimpleAction::new_stateful(&uuid_string_random(), None, &String::new().to_variant());
        action_link_bookmark.connect_activate({
            let p = profile.clone();
            move |this, _| {
                let state = this.state().unwrap().get::<String>().unwrap();
                p.bookmark.toggle(&state, None).unwrap();
            }
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
                g.add_action(&action_link_copy_url);
                g.add_action(&action_link_copy_text);
                g.add_action(&action_link_copy_text_selected);
                g.add_action(&action_link_bookmark);
                g.add_action(&action_link_download);
                g.add_action(&action_link_source);
                g
            }),
        );
        let link_context = gtk::PopoverMenu::from_model(Some(&{
            let m = Menu::new();
            m.append(
                Some("Open Link in New Tab"),
                Some(&format!(
                    "{link_context_group_id}.{}",
                    action_link_tab.name()
                )),
            );
            m.append_section(None, &{
                let m_copy = Menu::new();
                m_copy.append(
                    Some("Copy Link URL"),
                    Some(&format!(
                        "{link_context_group_id}.{}",
                        action_link_copy_url.name()
                    )),
                );
                m_copy.append(
                    Some("Copy Link Text"),
                    Some(&format!(
                        "{link_context_group_id}.{}",
                        action_link_copy_text.name()
                    )),
                );
                m_copy.append(
                    Some("Copy Text Selected"),
                    Some(&format!(
                        "{link_context_group_id}.{}",
                        action_link_copy_text_selected.name()
                    )),
                );
                m_copy
            });
            m.append_section(None, &{
                let m_other = Menu::new();
                m_other.append(
                    Some("Bookmark Link"), // @TODO highlight state
                    Some(&format!(
                        "{link_context_group_id}.{}",
                        action_link_bookmark.name()
                    )),
                );
                m_other.append(
                    Some("Download Link"),
                    Some(&format!(
                        "{link_context_group_id}.{}",
                        action_link_download.name()
                    )),
                );
                m_other.append(
                    Some("View Link as Source"),
                    Some(&format!(
                        "{link_context_group_id}.{}",
                        action_link_source.name()
                    )),
                );
                m_other
            });
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
                            let is_prefix_link = is_prefix_link(&request_str);

                            // Open in the new tab
                            action_link_tab.set_state(&request_var);
                            action_link_tab.set_enabled(!request_str.is_empty());

                            // Copy link to the clipboard
                            action_link_copy_url.set_state(&request_var);
                            action_link_copy_url.set_enabled(!request_str.is_empty());

                            // Copy link text
                            {
                                let mut start_iter = iter;
                                let mut end_iter = iter;
                                if !start_iter.starts_tag(Some(&tag)) {
                                    start_iter.backward_to_tag_toggle(Some(&tag));
                                }
                                if !end_iter.ends_tag(Some(&tag)) {
                                    end_iter.forward_to_tag_toggle(Some(&tag));
                                }
                                let tagged_text = text_view
                                    .buffer()
                                    .text(&start_iter, &end_iter, false)
                                    .replace(LINK_EXTERNAL_INDICATOR, "")
                                    .trim()
                                    .to_string();

                                action_link_copy_text.set_state(&tagged_text.to_variant());
                                action_link_copy_text.set_enabled(!tagged_text.is_empty());
                            }

                            // Copy link text (if) selected
                            action_link_copy_text_selected.set_enabled(
                                if let Some((start, end)) = buffer.selection_bounds() {
                                    let selected = buffer.text(&start, &end, false);
                                    action_link_copy_text_selected
                                        .set_state(&selected.to_variant());
                                    !selected.is_empty()
                                } else {
                                    false
                                },
                            );

                            // Bookmark
                            action_link_bookmark.set_state(&request_var);
                            action_link_bookmark.set_enabled(is_prefix_link);

                            // Download (new tab)
                            action_link_download.set_state(&request_var);
                            action_link_download.set_enabled(is_prefix_link);

                            // View as Source (new tab)
                            action_link_source.set_state(&request_var);
                            action_link_source.set_enabled(is_prefix_link);

                            // Toggle
                            link_context
                                .set_pointing_to(Some(&gtk::gdk::Rectangle::new(x, y, 1, 1)));
                            link_context.popup()
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
                "Invalid multiline markup! Gemtext format partially ignored.".to_string(),
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

fn is_prefix_link(request: &str) -> bool {
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

const LINK_EXTERNAL_INDICATOR: &str = "⇖";
const LINK_PREFIX_DOWNLOAD: &str = "download:";
const LINK_PREFIX_SOURCE: &str = "source:";
