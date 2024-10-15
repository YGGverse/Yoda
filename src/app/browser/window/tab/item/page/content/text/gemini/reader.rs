mod parser;
mod tag;
mod widget;

use parser::code::Code;
use parser::header::Header;
use parser::link::Link;
use parser::list::List;
use parser::quote::Quote;
use tag::Tag;
use widget::Widget;

use gtk::{
    gdk::{BUTTON_MIDDLE, BUTTON_PRIMARY},
    gio::{AppInfo, AppLaunchContext, SimpleAction},
    glib::{GString, TimeZone, Uri},
    prelude::{ActionExt, TextBufferExt, TextBufferExtManual, TextViewExt, ToVariant, WidgetExt},
    EventControllerMotion, GestureClick, TextBuffer, TextTag, TextView, TextWindowType, WrapMode,
};

use std::{collections::HashMap, sync::Arc};

pub struct Reader {
    title: Option<GString>,
    widget: Arc<Widget>,
}

impl Reader {
    // Construct
    pub fn new_arc(
        gemtext: &str,
        base: &Uri,
        action_tab_open: Arc<SimpleAction>,
        action_page_open: Arc<SimpleAction>,
    ) -> Arc<Self> {
        // Init default values
        let mut title = None;

        // Init HashMap storage for event controllers
        let mut links: HashMap<&TextTag, Uri> = HashMap::new();

        // Init multiline code builder features
        let mut multiline = None;

        // Init tags
        let tag = Tag::new();

        // Init new text buffer
        let buffer = TextBuffer::new(Some(tag.gobject()));

        // Parse gemtext lines
        for line in gemtext.lines() {
            // Is inline code
            if let Some(code) = Code::inline_from(line) {
                // Append value to buffer
                buffer.insert_with_tags(&mut buffer.end_iter(), code.value.as_str(), &[tag.code()]);
                buffer.insert(&mut buffer.end_iter(), "\n");

                // Skip other actions for this line
                continue;
            }

            // Is multiline code
            match multiline {
                None => {
                    // Open tag found
                    if let Some(code) = Code::multiline_begin_from(line) {
                        // Begin next lines collection into the code buffer
                        multiline = Some(code);

                        // Skip other actions for this line
                        continue;
                    }
                }
                Some(ref mut this) => {
                    Code::multiline_continue_from(this, line);

                    // Close tag found:
                    if this.completed {
                        // Is alt provided
                        if let Some(alt) = &this.alt {
                            // Insert alt value to the main buffer
                            buffer.insert_with_tags(
                                &mut buffer.end_iter(),
                                alt.as_str(),
                                &[tag.title()],
                            );
                            buffer.insert(&mut buffer.end_iter(), "\n");
                        }

                        // Insert multiline code buffer into main buffer
                        buffer.insert_with_tags(
                            &mut buffer.end_iter(),
                            &this.buffer.join("\n"),
                            &[tag.code()],
                        );

                        buffer.insert(&mut buffer.end_iter(), "\n");

                        // Reset
                        multiline = None;
                    }

                    // Skip other actions for this line
                    continue;
                }
            };

            // Is header
            if let Some(header) = Header::from(line) {
                // Append value to buffer
                buffer.insert_with_tags(
                    &mut buffer.end_iter(),
                    header.value.as_str(),
                    &[match header.level {
                        parser::header::Level::H1 => tag.h1(),
                        parser::header::Level::H2 => tag.h2(),
                        parser::header::Level::H3 => tag.h3(),
                    }],
                );
                buffer.insert(&mut buffer.end_iter(), "\n");

                // Update reader title using first gemtext header match
                if title == None {
                    title = Some(header.value.clone());
                }

                // Skip other actions for this line
                continue;
            }

            // Is link
            if let Some(link) = Link::from(line, Some(base), Some(&TimeZone::local())) {
                // Append tag to HashMap storage
                // links.insert(tag.link(), link.uri.clone()); @TODO

                // Create vector for alt values
                let mut alt = Vec::new();

                // Append external indicator on exist
                if let Some(is_external) = link.is_external {
                    if is_external {
                        alt.push("⇖".to_string());
                    }
                }

                // Append date on exist
                if let Some(timestamp) = link.timestamp {
                    // https://docs.gtk.org/glib/method.DateTime.format.html
                    if let Ok(value) = timestamp.format("%Y-%m-%d") {
                        alt.push(value.to_string())
                    }
                }

                // Append alt value on exist or use URL
                alt.push(match link.alt {
                    Some(alt) => alt.to_string(),
                    None => link.uri.to_string(),
                });

                // Append alt vector values to buffer
                buffer.insert_with_tags(&mut buffer.end_iter(), &alt.join(" "), &[&tag.link()]);
                buffer.insert(&mut buffer.end_iter(), "\n");

                // Skip other actions for this line
                continue;
            }

            // Is list
            if let Some(list) = List::from(line) {
                // Append value to buffer
                buffer.insert_with_tags(
                    &mut buffer.end_iter(),
                    format!("• {}", list.value).as_str(),
                    &[&tag.list()],
                );
                buffer.insert(&mut buffer.end_iter(), "\n");

                // Skip other actions for this line
                continue;
            }

            // Is quote
            if let Some(quote) = Quote::from(line) {
                // Append value to buffer
                buffer.insert_with_tags(
                    &mut buffer.end_iter(),
                    quote.value.as_str(),
                    &[&tag.quote()],
                );
                buffer.insert(&mut buffer.end_iter(), "\n");

                // Skip other actions for this line
                continue;
            }

            // Nothing match custom tags above,
            // just append plain text covered in empty tag (to handle controller events properly)
            let tag = TextTag::builder().wrap_mode(WrapMode::Word).build();

            buffer.tag_table().add(&tag);
            buffer.insert_with_tags(&mut buffer.end_iter(), &line, &[&tag]);
            buffer.insert(&mut buffer.end_iter(), "\n");
        }

        // Init additional controllers
        let primary_button_controller = GestureClick::builder().button(BUTTON_PRIMARY).build();
        let middle_button_controller = GestureClick::builder().button(BUTTON_MIDDLE).build();
        let motion_controller = EventControllerMotion::new();

        // Init widget
        let widget = Widget::new_arc(
            &buffer,
            primary_button_controller.clone(),
            middle_button_controller.clone(),
            motion_controller.clone(),
        );

        // Init events
        primary_button_controller.connect_released({
            let action_page_open = action_page_open.clone();
            let gobject = widget.gobject().clone();
            let _links_ = links.clone(); // is copy
            move |_, _, window_x, window_y| {
                // Detect tag match current coords hovered
                let (buffer_x, buffer_y) = gobject.window_to_buffer_coords(
                    TextWindowType::Widget,
                    window_x as i32,
                    window_y as i32,
                );

                if let Some(iter) = gobject.iter_at_location(buffer_x, buffer_y) {
                    for tag in iter.tags() {
                        // Tag is link
                        if let Some(uri) = _links_.get(&tag) {
                            // Select link handler by scheme
                            return match uri.scheme().as_str() {
                                "gemini" => {
                                    // Open new page in browser
                                    action_page_open.activate(Some(&uri.to_str().to_variant()));
                                }
                                // Scheme not supported, delegate link to the external app
                                _ => match AppInfo::launch_default_for_uri(
                                    &uri.to_str(),
                                    Some(&AppLaunchContext::new()),
                                ) {
                                    Ok(_) => (),
                                    Err(e) => todo!("{e}"),
                                },
                            };
                        }
                    }
                }
            }
        });

        middle_button_controller.connect_pressed({
            let gobject = widget.gobject().clone();
            let _links_ = links.clone(); // is copy
            move |_, _, window_x, window_y| {
                // Detect tag match current coords hovered
                let (buffer_x, buffer_y) = gobject.window_to_buffer_coords(
                    TextWindowType::Widget,
                    window_x as i32,
                    window_y as i32,
                );
                if let Some(iter) = gobject.iter_at_location(buffer_x, buffer_y) {
                    for tag in iter.tags() {
                        // Tag is link
                        if let Some(uri) = _links_.get(&tag) {
                            return action_tab_open.activate(Some(&uri.to_string().to_variant()));
                        }
                    }
                }
            }
        }); // for a note: this action sensitive to focus out

        motion_controller.connect_motion({
            let gobject = widget.gobject().clone();
            let _links_ = links.clone(); // is copy
            move |_, window_x, window_y| {
                // Detect tag match current coords hovered
                let (buffer_x, buffer_y) = gobject.window_to_buffer_coords(
                    TextWindowType::Widget,
                    window_x as i32,
                    window_y as i32,
                );

                if let Some(iter) = gobject.iter_at_location(buffer_x, buffer_y) {
                    for tag in iter.tags() {
                        // Tag is link
                        if let Some(uri) = _links_.get(&tag) {
                            // Toggle cursor
                            gobject.set_cursor_from_name(Some("pointer"));

                            // Show tooltip | @TODO set_gutter option?
                            gobject.set_tooltip_text(Some(uri.to_string().as_str()));

                            // Any signal required to apply changes immediately @TODO power safe issue?
                            gobject.emit_toggle_overwrite();

                            return;
                        }
                    }
                }

                // Restore defaults
                gobject.set_cursor_from_name(Some("text"));
                gobject.set_tooltip_text(None);
                gobject.emit_toggle_overwrite();
            }
        }); // @TODO may be expensive for CPU, add timeout?

        // Result
        Arc::new(Self { title, widget })
    }

    // Getters
    pub fn title(&self) -> &Option<GString> {
        &self.title
    }

    pub fn gobject(&self) -> &TextView {
        &self.widget.gobject()
    }
}
