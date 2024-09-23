pub struct Tab {
    tab: gtk::Notebook,
}

impl Tab {
    // Construct new object
    pub fn new() -> Tab {
        Self {
            tab: gtk::Notebook::builder().scrollable(true).build(),
        }
    }

    // Actions
    pub fn append(&self, label: &gtk::Box, page: &gtk::Box, current: bool) -> u32 {
        let page_number = self.tab.append_page(page, Some(label));

        self.tab.set_tab_reorderable(page, true);

        if current {
            self.tab.set_current_page(Some(page_number));
        }

        page_number
    }

    // Getters
    pub fn tab(&self) -> &gtk::Notebook {
        &self.tab
    }
}
