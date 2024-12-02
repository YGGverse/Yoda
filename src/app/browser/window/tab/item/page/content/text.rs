mod gemini;

use gemini::Gemini;

use crate::app::browser::window::{tab::item::Action as TabAction, Action as WindowAction};
use gtk::{
    glib::{GString, Uri},
    ScrolledWindow,
};
use std::rc::Rc;

pub struct Meta {
    pub title: Option<GString>,
} // @TODO move to separated mod

pub struct Text {
    pub meta: Meta,
    pub scrolled_window: ScrolledWindow,
}

impl Text {
    // Construct
    pub fn gemini(gemtext: &str, base: &Uri, actions: (Rc<WindowAction>, Rc<TabAction>)) -> Self {
        // Init components
        let gemini = Gemini::new(gemtext, base, actions);

        // Init meta
        let meta = Meta {
            title: gemini.reader.title.clone(),
        };

        // Init scrolled_window
        let scrolled_window = ScrolledWindow::builder().build();

        scrolled_window.set_child(Some(&gemini.widget.clamp_scrollable));

        // Result
        Self {
            meta,
            scrolled_window,
        }
    }
}
