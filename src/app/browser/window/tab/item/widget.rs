use adw::{TabPage, TabView};
use gtk::Box;
use std::sync::Arc;

pub struct Widget {
    gobject: TabPage,
}

impl Widget {
    // Construct
    pub fn new_arc(
        tab_view: &TabView,
        page: &Box,
        title: Option<&str>,
        is_selected_page: bool,
    ) -> Arc<Self> {
        let gobject = tab_view.append(page);

        if let Some(value) = title {
            gobject.set_title(value);
        }

        if is_selected_page {
            tab_view.set_selected_page(&gobject);
        }

        Arc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &TabPage {
        &self.gobject
    }
}
