mod gemini;
mod search;
mod source;

use gemini::Gemini;
use search::Search;
use source::Source;

use super::{TabAction, WindowAction};
use gtk::{
    glib::Uri,
    prelude::{BoxExt, ButtonExt, TextViewExt, WidgetExt},
    Box, Orientation, ScrolledWindow,
};
use std::rc::Rc;

pub struct Meta {
    pub title: Option<String>,
} // @TODO move to separated mod

pub struct Text {
    pub g_box: Box,
    pub has_search: bool,
    pub meta: Meta,
}

impl Text {
    // Constructors

    pub fn new_gemini(
        gemtext: &str,
        base: &Uri,
        (window_action, tab_action): (Rc<WindowAction>, Rc<TabAction>),
    ) -> Self {
        // Init components
        let gemini = Gemini::new(gemtext, base, (window_action.clone(), tab_action));
        let search = Rc::new(Search::new(&gemini.reader.buffer));

        // Init main widget
        let g_box = Box::builder().orientation(Orientation::Vertical).build();

        g_box.append(
            &ScrolledWindow::builder()
                .child(&gemini.widget.clamp_scrollable)
                .build(),
        );

        g_box.append(&search.g_box);

        // Connect events
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
                    text_view.scroll_to_iter(&mut start, 0.0, false, 0.0, 0.0);
                }
            }
        });

        search.navigation.forward.button.connect_clicked({
            let text_view = gemini.reader.widget.text_view.clone();
            let navigation = search.navigation.clone();
            move |_| {
                if let Some((mut start, _)) = navigation.forward() {
                    text_view.scroll_to_iter(&mut start, 0.0, false, 0.0, 0.0);
                }
            }
        });

        search.close.connect_clicked({
            let search = search.clone();
            move |_| {
                search.g_box.set_visible(false);
            }
        });

        Self {
            meta: Meta {
                title: gemini.reader.title.clone(),
            },
            has_search: true,
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
            has_search: false,
            g_box,
        }
    }
}
