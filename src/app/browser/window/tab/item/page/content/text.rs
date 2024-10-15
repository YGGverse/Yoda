mod gemini;

use gemini::Gemini;

use gtk::{
    gio::SimpleAction,
    glib::{GString, Uri},
    ScrolledWindow,
};

use std::sync::Arc;

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
        action_tab_append: Arc<SimpleAction>,
        action_page_open: Arc<SimpleAction>,
    ) -> Self {
        // Init components
        let gemini = Gemini::new(gemtext, base, action_tab_append, action_page_open);

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
