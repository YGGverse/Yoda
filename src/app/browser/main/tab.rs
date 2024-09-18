#[path = "tab/label.rs"] mod label;
#[path = "tab/page.rs"] mod page;

use gtk::Notebook;

pub fn new() -> Notebook
{
    let tab = Notebook::builder().scrollable(true).build();

    append(
        tab.clone(),
        true
    ); // @TODO

    return tab;
}

pub fn append(
    tab : Notebook,
    current : bool
) -> u32
{
    let page = page::new();

    let page_number = tab.append_page(
        &page,
        Some(
            &label::new()
        )
    );

    tab.set_tab_reorderable(
        &page,
        true
    );

    if current
    {
        tab.set_current_page(
            Some(
                page_number
            )
        );
    }

    return page_number;
}