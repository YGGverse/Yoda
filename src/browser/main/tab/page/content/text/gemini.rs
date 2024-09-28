mod reader;

use reader::Reader;

use gtk::{
    gio::SimpleAction,
    glib::{GString, Uri},
    Viewport,
};

use std::sync::Arc;

pub struct Gemini {
    reader: Reader,
    widget: Viewport,
}

impl Gemini {
    // Construct
    pub fn new(gemtext: &str, base: &Uri, action_open: Arc<SimpleAction>) -> Self {
        // Init components
        let reader = Reader::new(gemtext, base, action_open);

        // Init widget
        let widget = Viewport::builder().scroll_to_focus(false).build();

        widget.set_child(Some(reader.widget()));

        // Result
        Self { reader, widget }
    }

    // Getters
    pub fn reader_title(&self) -> &Option<GString> {
        &self.reader.title()
    }

    pub fn widget(&self) -> &Viewport {
        &self.widget
    }
}
