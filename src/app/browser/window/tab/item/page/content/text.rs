mod gemini;
mod source;

use gemini::Gemini;
use source::Source;

use crate::app::browser::window::{tab::item::Action as TabAction, Action as WindowAction};
use gtk::{glib::Uri, ScrolledWindow};
use std::rc::Rc;

pub struct Meta {
    pub title: Option<String>,
} // @TODO move to separated mod

pub struct Text {
    pub meta: Meta,
    pub scrolled_window: ScrolledWindow,
}

impl Text {
    // Constructors

    pub fn new_gemini(
        gemtext: &str,
        base: &Uri,
        actions: (Rc<WindowAction>, Rc<TabAction>),
    ) -> Self {
        // Init widget driver
        let gemini = Gemini::new(gemtext, base, actions);

        // Init scrolled container
        let scrolled_window = ScrolledWindow::builder().build();

        scrolled_window.set_child(Some(&gemini.widget.clamp_scrollable));

        // Result
        Self {
            meta: Meta {
                title: gemini.reader.title.clone(),
            },
            scrolled_window,
        }
    }

    pub fn new_source(data: &str) -> Self {
        // Init scrolled container
        let scrolled_window = ScrolledWindow::builder().build();
        scrolled_window.set_child(Some(&Source::new(data).text_view));

        // Result
        Self {
            meta: Meta { title: None },
            scrolled_window,
        }
    }
}
