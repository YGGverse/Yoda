#[path = "history/back.rs"]
mod back;
#[path = "history/forward.rs"]
mod forward;

use gtk::prelude::BoxExt;
use gtk::Box;

pub fn new() -> Box {
    let history = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .css_classes([
            "linked", // merge childs
        ])
        .build();

    // Compose childs
    history.append(&back::new());
    history.append(&forward::new());

    history
}
