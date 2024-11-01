mod gemini;

use gemini::Gemini;

use gtk::{
    gio::SimpleAction,
    glib::{GString, Uri},
    ScrolledWindow,
};

pub struct Meta {
    title: Option<GString>,
}

pub struct Text {
    meta: Meta,
    gobject: ScrolledWindow,
}

impl Text {
    // Construct
    pub fn gemini(
        gemtext: &str,
        base: &Uri,
        action_page_new: SimpleAction,
        action_page_open: SimpleAction,
    ) -> Self {
        // Init components
        let gemini = Gemini::new(gemtext, base, action_page_new, action_page_open);

        // Init meta
        let meta = Meta {
            title: gemini.reader_title().clone(),
        };

        // Init gobject
        let gobject = ScrolledWindow::builder().build();

        gobject.set_child(Some(gemini.gobject()));

        // Result
        Self { meta, gobject }
    }

    // Getters
    pub fn meta_title(&self) -> &Option<GString> {
        &self.meta.title
    }

    pub fn gobject(&self) -> &ScrolledWindow {
        &self.gobject
    }
}
