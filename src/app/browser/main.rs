#[path = "main/tab.rs"]
mod tab;

use gtk::prelude::BoxExt;
use gtk::Box;

pub fn new() -> Box {
    let main = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    main.append(&tab::new());

    main
}
