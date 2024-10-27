mod reader;
mod widget;

use reader::Reader;
use widget::Widget;

use gtk::{
    gio::SimpleAction,
    glib::{GString, Uri},
};

use adw::ClampScrollable;

use std::sync::Arc;

pub struct Gemini {
    reader: Arc<Reader>,
    widget: Arc<Widget>,
}

impl Gemini {
    // Construct
    pub fn new(
        gemtext: &str,
        base: &Uri,
        action_tab_open: SimpleAction,
        action_page_open: SimpleAction,
    ) -> Self {
        // Init components
        let reader = Reader::new_arc(gemtext, base, action_tab_open, action_page_open);

        let widget = Widget::new_arc(&reader.gobject());

        // Result
        Self { reader, widget }
    }

    // Getters
    pub fn reader_title(&self) -> &Option<GString> {
        &self.reader.title()
    }

    pub fn gobject(&self) -> &ClampScrollable {
        &self.widget.gobject()
    }
}
