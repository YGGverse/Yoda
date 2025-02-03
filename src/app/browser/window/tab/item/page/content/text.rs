mod gemini;
mod source;

use super::{ItemAction, WindowAction};
use adw::ClampScrollable;
use gemini::Gemini;
use gtk::{glib::Uri, prelude::Cast, ScrolledWindow, TextView};
use source::Source;
use std::rc::Rc;

pub struct Meta {
    pub title: Option<String>,
} // @TODO move to separated mod

pub struct Text {
    pub meta: Meta,
    pub scrolled_window: ScrolledWindow,
    pub text_view: TextView,
}

impl Text {
    pub fn gemini(
        actions: (&Rc<WindowAction>, &Rc<ItemAction>),
        base: &Uri,
        gemtext: &str,
    ) -> Self {
        let gemini = Gemini::build(actions, base, gemtext).unwrap(); // @TODO handle

        let clamp_scrollable = ClampScrollable::builder()
            .child(&gemini.text_view)
            .css_classes(["view"])
            .maximum_size(800)
            .build();

        Self {
            text_view: gemini.text_view,
            meta: Meta {
                title: gemini.title,
            },
            scrolled_window: ScrolledWindow::builder().child(&clamp_scrollable).build(),
        }
    }

    pub fn source(data: &str) -> Self {
        let source = Source::new(data);
        Self {
            scrolled_window: ScrolledWindow::builder().child(&source.text_view).build(),
            text_view: source.text_view.upcast::<TextView>(),
            meta: Meta { title: None },
        }
    }
}
