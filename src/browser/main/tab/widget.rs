pub struct Tab {
    notebook: gtk::Notebook,
}

impl Tab {
    // Construct new object
    pub fn new() -> Tab {
        Self {
            notebook: gtk::Notebook::builder().scrollable(true).build(),
        }
    }

    // Actions
    pub fn append(&self, label: &gtk::Box, page: &gtk::Box, current: bool) -> u32 {
        let page_number = self.notebook.append_page(page, Some(label));

        self.notebook.set_tab_reorderable(page, true);

        if current {
            self.notebook.set_current_page(Some(page_number));
        }

        page_number
    }

    // Getters
    pub fn notebook(&self) -> &gtk::Notebook {
        &self.notebook
    }
}
