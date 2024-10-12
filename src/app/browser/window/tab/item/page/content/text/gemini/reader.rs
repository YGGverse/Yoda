mod parser;
mod widget;

use parser::header::Header;
use parser::link::Link;
use widget::Widget;

use gtk::{
    gio::SimpleAction,
    glib::{GString, TimeZone, Uri},
    prelude::{TextBufferExt, TextBufferExtManual},
    TextBuffer, TextTag, TextTagTable, TextView, WrapMode,
};

use std::sync::Arc;

pub struct Reader {
    title: Option<GString>,
    // css: CssProvider,
    widget: Arc<Widget>,
}

impl Reader {
    // Construct
    pub fn new_arc(gemtext: &str, base: &Uri, action_page_open: Arc<SimpleAction>) -> Arc<Self> {
        // Init title
        let mut title = None;

        // Init tag table
        let tags = TextTagTable::new();

        // Init header tags
        let h1 = TextTag::builder()
            .name("h1")
            .scale(1.6)
            .weight(500)
            .wrap_mode(gtk::WrapMode::Word)
            .build();

        tags.add(&h1);

        let h2 = TextTag::builder()
            .name("h2")
            .scale(1.4)
            .weight(400)
            .wrap_mode(gtk::WrapMode::Word)
            .build();

        tags.add(&h2);

        let h3 = TextTag::builder()
            .name("h3")
            .scale(1.2)
            .weight(400)
            .wrap_mode(WrapMode::Word)
            .build();

        tags.add(&h3);

        // Init link tag
        let link = TextTag::builder()
            .name("link")
            .wrap_mode(WrapMode::Word)
            .build();

        tags.add(&link);

        // Parse lines
        let buffer = TextBuffer::new(Some(&tags));

        for line in gemtext.lines() {
            // Is header
            if let Some(header) = Header::from(line) {
                // Detect level
                let tag = match header.level {
                    parser::header::Level::H1 => "h1",
                    parser::header::Level::H2 => "h2",
                    parser::header::Level::H3 => "h3",
                };

                // Insert tag line
                buffer.insert_with_tags_by_name(
                    &mut buffer.end_iter(),
                    header.value.as_str(),
                    &[tag],
                );

                buffer.insert(&mut buffer.end_iter(), "\n");

                // Set title if empty, on first document header match
                // this feature wanted to update parent elements like tab title
                if title == None {
                    title = Some(header.value.clone());
                }

                continue;
            }

            // Is link
            if let Some(link) = Link::from(line, Some(base), Some(&TimeZone::local())) {
                // Build link alt from optional values
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

                // Append alt on exist or use URL
                alt.push(match link.alt {
                    Some(alt) => alt.to_string(),
                    None => link.uri.to_string(),
                });

                buffer.insert_with_tags_by_name(&mut buffer.end_iter(), &alt.join(" "), &["link"]);
                buffer.insert(&mut buffer.end_iter(), "\n");

                continue;
            }

            // Nothing match, use plain text @TODO
            buffer.insert(&mut buffer.end_iter(), line);
            buffer.insert(&mut buffer.end_iter(), "\n");
        }

        // Init widget
        let widget = Widget::new_arc(&buffer);

        // Connect actions
        /* @TODO
        widget.connect_activate_link(move |_, href| {
            // Detect requested protocol
            if let Ok(uri) = Uri::parse(&href, UriFlags::NONE) {
                return match uri.scheme().as_str() {
                    "gemini" => {
                        // Open new page
                        action_page_open.activate(Some(&uri.to_str().to_variant()));

                        // Prevent link open in external application
                        Propagation::Stop
                    }
                    // Protocol not supported
                    _ => Propagation::Proceed,
                };
            }

            // Delegate unparsable
            Propagation::Proceed
        }); */

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
