mod parser;
mod widget;

use parser::header::Header;
use parser::link::Link;
use widget::Widget;

use gtk::{
    gio::SimpleAction,
    glib::{GString, Uri},
    prelude::TextBufferExt,
    TextBuffer, TextView,
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

        // Init markup
        let buffer = TextBuffer::new(None);

        // Parse lines
        for line in gemtext.lines() {
            /*
            // Is header
            if let Some(header) = Header::from(line) {
                // Format
                buffer.insert_markup(&mut buffer.end_iter(), header.markup());

                // Set title from first document header tag
                if title == None {
                    title = Some(header.text().clone());
                }

                continue;
            }

            // Is link
            if let Some(link) = Link::from(line, base) {
                // Format
                buffer.insert_markup(&mut buffer.end_iter(), link.markup());

                continue;
            }

            // Nothing match, escape string just
            buffer.insert_markup(&mut buffer.end_iter(), Plain::from(line).markup()) */

            buffer.insert(&mut buffer.end_iter(), format!("{line}\n").as_str()) // @TODO
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
