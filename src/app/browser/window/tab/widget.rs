use adw::TabView;
use gtk::glib::GString;

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
    pub fn current_page_keyword(&self) -> Option<GString> {
        let page = self.gobject.selected_page()?;
        let id = page.keyword()?;
        Some(id)
    }

    pub fn gobject(&self) -> &TabView {
        &self.gobject
    }
}
