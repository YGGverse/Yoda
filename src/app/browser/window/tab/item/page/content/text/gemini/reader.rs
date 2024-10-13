mod parser;
mod widget;

use parser::header::Header;
use parser::link::Link;
use widget::Widget;

use gtk::{
    gdk::{BUTTON_MIDDLE, BUTTON_PRIMARY},
    gio::SimpleAction,
    glib::{GString, TimeZone, Uri},
    prelude::{ActionExt, TextBufferExt, TextBufferExtManual, TextViewExt, ToVariant},
    EventControllerMotion, GestureClick, TextBuffer, TextTag, TextView, TextWindowType, WrapMode,
};

use std::{collections::HashMap, sync::Arc};

pub struct Reader {
    title: Option<GString>,
    widget: Arc<Widget>,
}

impl Reader {
    // Construct
    pub fn new_arc(gemtext: &str, base: &Uri, action_page_open: Arc<SimpleAction>) -> Arc<Self> {
        // Init default values
        let mut title = None;

        // Init HashMap storage for event controllers
        let mut links: HashMap<TextTag, Uri> = HashMap::new();

        // Init new text buffer
        let buffer = TextBuffer::new(None);

        // Parse gemtext lines
        for line in gemtext.lines() {
            // Is header
            if let Some(header) = Header::from(line) {
                // Build tag from level parsed
                let tag = match header.level {
                    parser::header::Level::H1 => TextTag::builder()
                        .scale(1.6)
                        .weight(500)
                        .wrap_mode(gtk::WrapMode::Word)
                        .build(),
                    parser::header::Level::H2 => TextTag::builder()
                        .scale(1.4)
                        .weight(400)
                        .wrap_mode(gtk::WrapMode::Word)
                        .build(),
                    parser::header::Level::H3 => TextTag::builder()
                        .scale(1.2)
                        .weight(400)
                        .wrap_mode(WrapMode::Word)
                        .build(),
                };

                // Register tag in buffer
                buffer.tag_table().add(&tag);

                // Append value to buffer
                buffer.insert_with_tags(&mut buffer.end_iter(), header.value.as_str(), &[&tag]);
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
                // Init new tag for link
                let tag = TextTag::builder().wrap_mode(WrapMode::Word).build();

                // Append tag to buffer
                buffer.tag_table().add(&tag);

                // Append tag to HashMap storage
                links.insert(tag.clone(), link.uri.clone());

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
                buffer.insert_with_tags(&mut buffer.end_iter(), &alt.join(" "), &[&tag]);
                buffer.insert(&mut buffer.end_iter(), "\n");

                // Skip other actions for this line
                continue;
            }

            // Nothing match tags, use plain text @TODO
            buffer.insert(&mut buffer.end_iter(), line);
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
            move |_, _, x, y| {
                gobject.window_to_buffer_coords(TextWindowType::Widget, x as i32, y as i32);
                if let Some(iter) = gobject.iter_at_location(x as i32, y as i32) {
                    for tag in iter.tags() {
                        if let Some(uri) = links.get(&tag) {
                            match uri.scheme().as_str() {
                                "gemini" => {
                                    // Open new page
                                    action_page_open.activate(Some(&uri.to_str().to_variant()));
                                }
                                _ => (), // Protocol not supported, delegate to the external app @TODO
                            };
                        }
                    }
                }
            }
        });

        // @TODO on middle-click, on hover events
        // primary_button_controller(|_, _, _, _| {});
        // motion_controller.connect_motion(|_, _, _| {});

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
