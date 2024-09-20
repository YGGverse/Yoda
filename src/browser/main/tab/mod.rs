mod label;
mod page;

use gtk::Notebook;

pub fn new() -> Notebook {
    let tab = Notebook::builder().scrollable(true).build();

    // Add test tab @TODO restore from session
    append(&tab, true);

    tab
}

pub fn append(tab: &Notebook, current: bool) -> u32 {
    let page = page::new();

    let page_number = tab.append_page(&page, Some(&label::new()));

    tab.set_tab_reorderable(&page, true);

    if current {
        tab.set_current_page(Some(page_number));
    }

    page_number
}
