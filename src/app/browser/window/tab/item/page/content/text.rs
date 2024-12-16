mod gemini;
mod source;

use gemini::Gemini;
use source::Source;

use crate::app::browser::window::{tab::item::Action as TabAction, Action as WindowAction};
use gtk::{glib::Uri, prelude::BoxExt, Box, Orientation, ScrolledWindow};
use std::rc::Rc;

pub struct Meta {
    pub title: Option<String>,
} // @TODO move to separated mod

pub struct Text {
    pub meta: Meta,
    pub g_box: Box,
}

impl Text {
    // Constructors

    pub fn new_gemini(
        gemtext: &str,
        base: &Uri,
        actions: (Rc<WindowAction>, Rc<TabAction>),
    ) -> Self {
        // Init components
        let gemini = Gemini::new(gemtext, base, actions);

        // Init main widget
        let g_box = Box::builder().orientation(Orientation::Vertical).build();
        g_box.append(
            &ScrolledWindow::builder()
                .child(&gemini.widget.clamp_scrollable)
                .build(),
        );

        Self {
            meta: Meta {
                title: gemini.reader.title.clone(),
            },
            g_box,
        }
    }

    pub fn new_source(data: &str) -> Self {
        let g_box = Box::builder().orientation(Orientation::Vertical).build();

        g_box.append(
            &ScrolledWindow::builder()
                .child(&Source::new(data).text_view)
                .build(),
        );

        Self {
            meta: Meta { title: None },
            g_box,
        }
    }
}
