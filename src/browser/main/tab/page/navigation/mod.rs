mod base;
mod bookmark;
mod history;
mod reload;
mod request;

use gtk::prelude::BoxExt;
use gtk::Box;

pub fn new() -> Box {
    let navigation = Box::builder()
        // Tuneup
        .orientation(gtk::Orientation::Horizontal)
        .spacing(8)
        .margin_top(8)
        .margin_start(8)
        .margin_end(8)
        .margin_bottom(8)
        .build();

    // Compose childs
    navigation.append(&base::new());
    navigation.append(&history::new());
    navigation.append(&reload::new());
    navigation.append(&request::new());
    navigation.append(&bookmark::new());

    navigation
}
