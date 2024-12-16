mod gemini;
mod source;

use gemini::Gemini;
use source::Source;

use super::{BrowserAction, TabAction, WindowAction};
use gtk::{
    glib::Uri,
    prelude::{BoxExt, TextViewExt},
    Box, Orientation, ScrolledWindow, TextBuffer,
};
use std::rc::Rc;

pub struct Meta {
    pub title: Option<String>,
} // @TODO move to separated mod

pub struct Text {
    pub buffer: TextBuffer,
    pub g_box: Box,
    pub meta: Meta,
}

impl Text {
    // Constructors

    pub fn new_gemini(
        gemtext: &str,
        base: &Uri,
        (browser_action, window_action, tab_action): (
            &Rc<BrowserAction>,
            &Rc<WindowAction>,
            &Rc<TabAction>,
        ),
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

        // Connect events
        /* @TODO
        browser_action.escape.connect_activate({
            let close = search.close.clone();
            move || {
                close.activate();
            }
        });

        window_action.find.connect_activate({
            let search = search.clone();
            move |_| {
                search.g_box.set_visible(true);
                search.input.entry.grab_focus();
            }
        });

        search.navigation.back.button.connect_clicked({
            let text_view = gemini.reader.widget.text_view.clone();
            let navigation = search.navigation.clone();
            move |_| {
                if let Some((mut start, _)) = navigation.back() {
                    text_view.scroll_to_iter(&mut start, 0.0, true, 0.0, 0.0);
                }
            }
        });

        search.navigation.forward.button.connect_clicked({
            let text_view = gemini.reader.widget.text_view.clone();
            let navigation = search.navigation.clone();
            move |_| {
                if let Some((mut start, _)) = navigation.forward() {
                    text_view.scroll_to_iter(&mut start, 0.0, true, 0.0, 0.0);
                }
            }
        });

        search.close.connect_clicked({
            let search = search.clone();
            move |_| {
                search.g_box.set_visible(false);
            }
        });*/

        Self {
            buffer: gemini.reader.widget.text_view.buffer(),
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
            buffer: source.text_view.buffer(),
            meta: Meta { title: None },
            g_box,
        }
    }
}
