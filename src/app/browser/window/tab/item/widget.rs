use adw::{TabPage, TabView};
use gtk::Box;
use std::sync::Arc;

pub struct Widget {
    gobject: TabPage,
}

impl Widget {
    // Construct
    pub fn new_arc(tab_view: &TabView, page: &Box, title: Option<&str>) -> Arc<Self> {
        let gobject = tab_view.append(page);

        if let Some(value) = title {
            gobject.set_title(value);
        }

        Arc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &TabPage {
        &self.gobject
    }
}
