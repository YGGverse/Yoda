mod gemini;
mod source;

use gemini::Gemini;
use source::Source;

use super::{TabAction, WindowAction};
use gtk::{
    glib::Uri,
    prelude::{BoxExt, Cast},
    Box, Orientation, ScrolledWindow, TextView,
};
use std::rc::Rc;

pub struct Meta {
    pub title: Option<String>,
} // @TODO move to separated mod

pub struct Text {
    pub text_view: TextView,
    pub g_box: Box,
    pub meta: Meta,
}

impl Text {
    // Constructors

    pub fn new_gemini(
        gemtext: &str,
        base: &Uri,
        (window_action, tab_action): (&Rc<WindowAction>, &Rc<TabAction>),
    ) -> Self {
        // Init components
        let gemini = Gemini::new(gemtext, base, (window_action, tab_action));

        // Init main widget
        let g_box = Box::builder().orientation(Orientation::Vertical).build();

        g_box.append(
            &ScrolledWindow::builder()
                .child(&gemini.widget.clamp_scrollable)
                .build(),
        );

        Self {
            text_view: gemini.reader.widget.text_view.clone().upcast::<TextView>(),
            meta: Meta {
                title: gemini.reader.title.clone(),
            },
            g_box,
        }
    }

    pub fn new_source(data: &str) -> Self {
        // Init components
        let source = Source::new(data);

        let g_box = Box::builder().orientation(Orientation::Vertical).build();

        g_box.append(&ScrolledWindow::builder().child(&source.text_view).build());

        Self {
            text_view: source.text_view.upcast::<TextView>(),
            meta: Meta { title: None },
            g_box,
        }
    }
}
