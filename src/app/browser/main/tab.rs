#[path = "tab/label.rs"] mod label;
#[path = "tab/page.rs"] mod page;

use gtk::Notebook;

pub fn new() -> Notebook
{
    return Notebook::builder().scrollable(true).build();
}

pub fn append(
    tab : Notebook,
    is_current : bool
) -> u32
{
    return tab.append_page(
        &page::new(),
        Some(
            &label::new()
        )
    );
}