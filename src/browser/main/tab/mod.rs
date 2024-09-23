mod label;
mod page;

use gtk::Notebook;
use label::Label;
use page::Page;

pub struct Tab {
    widget: Notebook,
}

impl Tab {
    // Construct
    pub fn new() -> Tab {
        Self {
            widget: Notebook::builder().scrollable(true).build(),
        }
    }

    // Actions
    pub fn append(&self, is_active: bool) -> u32 {
        let label = Label::new(false);
        let page = Page::new();

        let page_number = self.widget.append_page(page.widget(), Some(label.widget()));

        self.widget.set_tab_reorderable(page.widget(), true);

        if is_active {
            self.widget.set_current_page(Some(page_number));
        }

        page_number
    }

    pub fn close(&self) {
        todo!()
    }

    /* @TODO
    pub fn close_all(&self) {
        todo!()
    }*/

    pub fn pin(&self) -> bool {
        todo!()
    }

    // Getters
    pub fn widget(&self) -> &Notebook {
        &self.widget
    }
}
