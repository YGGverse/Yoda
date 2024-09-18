use gtk::Notebook;

pub fn new() -> Notebook
{
    return Notebook::builder().scrollable(true).build();
}