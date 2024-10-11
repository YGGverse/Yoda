use adw::{TabPage, TabView};
use gtk::Box;
use std::sync::Arc;

const DEFAULT_TITLE: &str = "New page";

pub struct Widget {
    gobject: TabPage,
}

impl Widget {
    // Construct
    pub fn new_arc(
        tab_view: &TabView,
        page: &Box,
        title: Option<&str>,
        is_selected: bool,
    ) -> Arc<Self> {
        let gobject = tab_view.append(page);

        gobject.set_title(match title {
            Some(value) => value,
            None => DEFAULT_TITLE,
        });

        if is_selected {
            tab_view.set_selected_page(&gobject);
        }

        Arc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &TabPage {
        &self.gobject
    }
}
