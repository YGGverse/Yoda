mod label;
mod page;

use std::sync::Arc;

use gtk::Notebook;
pub struct Tab {
    pub widget: Arc<gtk::Notebook>,
}

impl Tab {
    pub fn append(&self, current: bool) -> u32 {
        let page = page::new();

        let page_number = self.widget.append_page(&page, Some(&label::new()));

        self.widget.set_tab_reorderable(&page, true);

        if current {
            self.widget.set_current_page(Some(page_number));
        }

        page_number
    }
}

pub fn new() -> Tab {
    let widget = Arc::new(Notebook::builder().scrollable(true).build());

    Tab { widget }
}
