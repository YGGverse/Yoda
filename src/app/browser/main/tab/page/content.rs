use gtk::Box;
// use gtk::prelude::BoxExt; @TODO append

pub fn new() -> Box {
    Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build()
}
