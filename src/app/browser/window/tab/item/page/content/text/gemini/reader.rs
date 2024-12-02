pub mod error;
mod tag;
mod widget;

pub use error::Error;
use tag::Tag;
use widget::Widget;

use crate::app::browser::window::{
    action::Position, tab::item::Action as TabAction, Action as WindowAction,
};
use adw::StyleManager;
use gemtext::line::{
    code::Code,
    header::{Header, Level},
    link::Link,
    list::List,
    quote::Quote,
};
use gtk::{
    gdk::{BUTTON_MIDDLE, BUTTON_PRIMARY},
    gio::Cancellable,
    glib::{GString, TimeZone, Uri},
    prelude::{TextBufferExt, TextBufferExtManual, TextViewExt, WidgetExt},
    EventControllerMotion, GestureClick, TextBuffer, TextTag, TextWindowType, UriLauncher, Window,
    WrapMode,
};
use std::{collections::HashMap, rc::Rc};

pub struct Reader {
    pub title: Option<GString>,
    pub widget: Rc<Widget>,
}

impl Reader {
    // Construct
    pub fn new(
        gemtext: &str,
        base: &Uri,
        actions: (Rc<WindowAction>, Rc<TabAction>),
    ) -> Result<Self, Error> {
        // Init default values
        let mut title = None;

        // Init HashMap storage for event controllers
        let mut links: HashMap<TextTag, Uri> = HashMap::new();

        // Init multiline code builder features
        let mut multiline = None;

        // Init tags
        let tag = Tag::new();

        // Init new text buffer
        let buffer = TextBuffer::new(Some(&tag.text_tag_table));

        // Parse gemtext lines
        for line in gemtext.lines() {
            // Is inline code
            if let Some(code) = Code::inline_from(line) {
                // Append value to buffer
                buffer.insert_with_tags(
                    &mut buffer.end_iter(),
                    code.value.as_str(),
                    &[&tag.code.text_tag],
                );
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
                    match Code::multiline_continue_from(this, line) {
                        Ok(()) => {
                            // Close tag found:
                            if this.completed {
                                // Is alt provided
                                if let Some(alt) = &this.alt {
                                    // Insert alt value to the main buffer
                                    buffer.insert_with_tags(
                                        &mut buffer.end_iter(),
                                        alt.as_str(),
                                        &[&tag.title.text_tag],
                                    );
                                    buffer.insert(&mut buffer.end_iter(), "\n");
                                }

                                // Insert multiline code buffer into main buffer
                                buffer.insert_with_tags(
                                    &mut buffer.end_iter(),
                                    &this.buffer.join("\n"),
                                    &[&tag.code.text_tag],
                                );

                                buffer.insert(&mut buffer.end_iter(), "\n");

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
                    header.value.as_str(),
                    &[match header.level {
                        Level::H1 => &tag.h1.text_tag,
                        Level::H2 => &tag.h2.text_tag,
                        Level::H3 => &tag.h3.text_tag,
                    }],
                );
                buffer.insert(&mut buffer.end_iter(), "\n");

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

                // Create new tag for new link
                let a = TextTag::builder()
                    .foreground_rgba(&StyleManager::default().accent_color_rgba()) // @TODO
                    .sentence(true)
                    .wrap_mode(WrapMode::Word)
                    .build();

                if !tag.add(&a) {
                    panic!() // @TODO handle
                }

                // Append alt vector values to buffer
                buffer.insert_with_tags(&mut buffer.end_iter(), &alt.join(" "), &[&a]);
                buffer.insert(&mut buffer.end_iter(), "\n");

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
                    format!("• {}", list.value).as_str(),
                    &[&tag.list.text_tag],
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
                    &[&tag.quote.text_tag],
                );
                buffer.insert(&mut buffer.end_iter(), "\n");

                // Skip other actions for this line
                continue;
            }

            // Nothing match custom tags above,
            // just append plain text covered in empty tag (to handle controller events properly)
            let tag = TextTag::builder().wrap_mode(WrapMode::Word).build();

            buffer.tag_table().add(&tag);
            buffer.insert_with_tags(&mut buffer.end_iter(), line, &[&tag]);
            buffer.insert(&mut buffer.end_iter(), "\n");
        }

        // Init additional controllers
        let primary_button_controller = GestureClick::builder().button(BUTTON_PRIMARY).build();
        let middle_button_controller = GestureClick::builder().button(BUTTON_MIDDLE).build();
        let motion_controller = EventControllerMotion::new();

        // Init widget
        let widget = Rc::new(Widget::new(
            &buffer,
            primary_button_controller.clone(),
            middle_button_controller.clone(),
            motion_controller.clone(),
        ));

        // Init events
        primary_button_controller.connect_released({
            let text_view = widget.text_view.clone();
            let _links_ = links.clone(); // is copy
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
                        if let Some(uri) = _links_.get(&tag) {
                            // Select link handler by scheme
                            return match uri.scheme().as_str() {
                                "gemini" => {
                                    // Open new page in browser
                                    actions.1.load().activate(Some(&uri.to_str()), true);
                                }
                                // Scheme not supported, delegate
                                _ => UriLauncher::new(&uri.to_str()).launch(
                                    None::<&Window>,
                                    None::<&Cancellable>,
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
            let text_view = widget.text_view.clone();
            let _links_ = links.clone(); // is copy
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
                        if let Some(uri) = _links_.get(&tag) {
                            // Select link handler by scheme
                            return match uri.scheme().as_str() {
                                "gemini" => {
                                    // Open new page in browser
                                    actions.0.append.activate_stateful_once(
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
                                    None::<&Window>,
                                    None::<&Cancellable>,
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
        }); // for a note: this action sensitive to focus out

        motion_controller.connect_motion({
            let text_view = widget.text_view.clone();
            let _links_ = links.clone(); // is copy
            move |_, window_x, window_y| {
                // Detect tag match current coords hovered
                let (buffer_x, buffer_y) = text_view.window_to_buffer_coords(
                    TextWindowType::Widget,
                    window_x as i32,
                    window_y as i32,
                );

                if let Some(iter) = text_view.iter_at_location(buffer_x, buffer_y) {
                    for tag in iter.tags() {
                        // Tag is link
                        if let Some(uri) = _links_.get(&tag) {
                            // Toggle cursor
                            text_view.set_cursor_from_name(Some("pointer"));

                            // Show tooltip | @TODO set_gutter option?
                            text_view.set_tooltip_text(Some(uri.to_string().as_str()));

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
        Ok(Self { title, widget })
    }
}
