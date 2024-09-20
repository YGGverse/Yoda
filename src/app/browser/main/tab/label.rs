#[path = "label/pin.rs"]
mod pin;
#[path = "label/title.rs"]
mod title;

use gtk::prelude::BoxExt;
use gtk::Box;

pub fn new() -> Box {
    let label = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    label.append(&pin::new(false));
    label.append(&title::new());

    label
}
