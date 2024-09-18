use gtk4 as gtk;
use gtk::HeaderBar;

pub fn new() -> HeaderBar
{
    return HeaderBar::builder().build();
}