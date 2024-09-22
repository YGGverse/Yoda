pub struct Tab {
    gtk: gtk::Notebook,
}

impl Tab {
    // Construct new object
    pub fn new() -> Tab {
        Self {
            gtk: gtk::Notebook::builder().scrollable(true).build(),
        }
    }

    // Actions
    pub fn append(&self, label: &gtk::Box, page: &gtk::Box, current: bool) -> u32 {
        let page_number = self.gtk.append_page(page, Some(label));

        self.gtk.set_tab_reorderable(page, true);

        if current {
            self.gtk.set_current_page(Some(page_number));
        }

        page_number
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Notebook {
        &self.gtk
    }
}
