mod description;
mod title;

use gtk::prelude::BoxExt;
use gtk::Box;

pub fn new() -> Box {
    let subject = Box::builder()
        // Tuneup
        .orientation(gtk::Orientation::Vertical)
        .valign(gtk::Align::Center)
        .build();

    // Compose childs
    subject.append(&title::new());
    subject.append(&description::new());

    // Done
    subject
}
