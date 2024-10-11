use adw::{TabPage, TabView};
use gtk::{glib::GString, Box};

pub struct Widget {
    gobject: TabView,
}

impl Widget {
    // Construct
    pub fn new() -> Self {
        Self {
            gobject: TabView::builder().build(),
        }
    }

    // Actions
    pub fn close(&self) {
        if let Some(selected_page) = self.gobject.selected_page() {
            self.gobject.close_page(&selected_page);
        }
    }

    pub fn close_all(&self) {
        // @TODO skip pinned or make confirmation alert (GTK>=4.10)
        if let Some(selected_page) = self.gobject.selected_page() {
            self.gobject.close_other_pages(&selected_page);
            self.close();
        }
    }

    // Getters
    pub fn current_name(&self) -> Option<GString> {
        let page = self.gobject.selected_page()?;

        /* @TODO
        let widget_name = page.widget_name();
        if !widget_name.is_empty() {
            Some(widget_name)
        } else {
            None
        } */
        None
    }

    pub fn gobject(&self) -> &TabView {
        &self.gobject
    }
}
