mod content;
mod navigation;

use gtk::prelude::BoxExt;
use gtk::Box;

pub fn new() -> Box {
    let page = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    page.append(&navigation::new());
    page.append(&content::new());

    page
}