mod gutter;
mod tags;

use super::{ItemAction, WindowAction};
use crate::{app::browser::window::action::Position, profile::Profile};
use gtk::{
    EventControllerMotion, GestureClick, TextBuffer, TextSearchFlags, TextTag, TextTagTable,
    TextView, TextWindowType, UriLauncher, Window, WrapMode,
    gdk::{BUTTON_MIDDLE, BUTTON_PRIMARY, BUTTON_SECONDARY, Display, RGBA},
    gio::{Cancellable, Menu, SimpleAction, SimpleActionGroup},
    glib::{ControlFlow, GString, Uri, idle_add_local, uri_unescape_string, uuid_string_random},
    prelude::{PopoverExt, TextBufferExt, TextTagExt, TextViewExt, WidgetExt},
};
use gutter::Gutter;
use sourceview::prelude::{ActionExt, ActionMapExt, DisplayExt, ToVariant};
use std::{cell::Cell, collections::HashMap, rc::Rc};
use tags::Tags;

pub struct Markdown {
    pub title: Option<String>,
    pub text_view: TextView,
}

impl Markdown {
    // Constructors

    /// Build new `Self`
    pub fn build(
        (window_action, item_action): (&Rc<WindowAction>, &Rc<ItemAction>),
        profile: &Rc<Profile>,
        base: &Uri,
        markdown: &str,
    ) -> Self {
        // Init HashMap storage (for event controllers)
        let mut links: HashMap<TextTag, Uri> = HashMap::new();

        // Init hovered tag storage for `links`
        // * maybe less expensive than update entire HashMap by iter
        let hover: Rc<Cell<Option<TextTag>>> = Rc::new(Cell::new(None));

        // Init code features
        //let mut code = None;

        // Init colors
        // @TODO use accent colors in adw 1.6 / ubuntu 24.10+
        let link_color = (
            RGBA::new(0.208, 0.518, 0.894, 1.0),
            RGBA::new(0.208, 0.518, 0.894, 0.9),
        );

        // Init tags
        let mut tags = Tags::new();

        // Init new text buffer
        let buffer = TextBuffer::new(Some(&TextTagTable::new()));
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

        // Render markdown tags
        let title = tags.render(&buffer, base, &link_color.0, &mut links);

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
                    Some("Copy Link Text Selected"),
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
                            return if let Some(fragment) = uri.fragment() {
                                scroll_to_anchor(&text_view, fragment);
                            } else {
                                open_link_in_current_tab(&uri.to_string(), &item_action);
                            };
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

                            // Open in the new tab
                            action_link_tab.set_state(&request_var);
                            action_link_copy_text.set_enabled(!request_str.is_empty());

                            action_link_copy_url.set_state(&request_var);
                            action_link_copy_text.set_enabled(!request_str.is_empty());

                            {
                                // Copy link text
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
                            if let Some((sel_start, sel_end)) = buffer.selection_bounds() {
                                let selected_tag_text = buffer.text(&sel_start, &sel_end, false);
                                action_link_copy_text_selected
                                    .set_state(&selected_tag_text.to_variant());
                                action_link_copy_text_selected
                                    .set_enabled(!selected_tag_text.is_empty());
                            } else {
                                action_link_copy_text_selected.set_enabled(false);
                            }

                            // Bookmark
                            action_link_bookmark.set_state(&request_var);
                            action_link_bookmark.set_enabled(is_prefixable_link(&request_str));

                            // Download (new tab)
                            action_link_download.set_state(&request_var);
                            action_link_download.set_enabled(is_prefixable_link(&request_str));

                            // View as Source (new tab)
                            action_link_source.set_state(&request_var);
                            action_link_source.set_enabled(is_prefixable_link(&request_str));

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

        // Anchor auto-scroll behavior
        idle_add_local({
            let base = base.clone();
            let text_view = text_view.clone();
            move || {
                if let Some(fragment) = base.fragment() {
                    scroll_to_anchor(&text_view, fragment);
                }
                ControlFlow::Break
            }
        });

        Self { text_view, title }
    }
}

fn scroll_to_anchor(text_view: &TextView, fragment: GString) -> bool {
    fn try_scroll(text_view: &TextView, query: &str) -> bool {
        let mut cursor = text_view.buffer().start_iter();
        while let Some((mut match_start, match_end)) =
            cursor.forward_search(query, TextSearchFlags::CASE_INSENSITIVE, None)
        {
            if match_start
                .tags()
                .iter()
                .any(|t| t.name().is_some_and(|n| n.starts_with("h")))
            {
                return text_view.scroll_to_iter(&mut match_start, 0.0, true, 0.0, 0.0);
            }
            cursor = match_end;
        }
        false
    }
    let query = uri_unescape_string(&fragment, None::<&str>).unwrap_or(fragment);
    let result = try_scroll(text_view, &query); // exact match
    if !result {
        return try_scroll(text_view, &query.replace("-", " ")); // unstable @TODO
    }
    result
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

const LINK_EXTERNAL_INDICATOR: &str = "⇖";
const LINK_PREFIX_DOWNLOAD: &str = "download:";
const LINK_PREFIX_SOURCE: &str = "source:";
