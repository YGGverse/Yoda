mod gemini;

use gemini::Gemini;

use gtk::{
    glib::{GString, Uri},
    ScrolledWindow,
};

pub struct Meta {
    title: Option<GString>,
}

pub struct Text {
    meta: Meta,
    widget: ScrolledWindow,
}

impl Text {
    // Construct
    pub fn gemini(gemtext: &str, base: &Uri) -> Self {
        // Init components
        let gemini = Gemini::new(gemtext, base);

        // Init meta
        let meta = Meta {
            title: gemini.reader_title().clone(),
        };

        // Init widget
        let widget = ScrolledWindow::builder().build();

        widget.set_child(Some(gemini.widget()));

        // Result
        Self { meta, widget }
    }

    // Getters
    pub fn meta_title(&self) -> &Option<GString> {
        &self.meta.title
    }

    pub fn widget(&self) -> &ScrolledWindow {
        &self.widget
    }
}
