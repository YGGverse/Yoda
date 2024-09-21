mod menu;
mod tab;

use gtk::prelude::BoxExt;
use gtk::Box;

pub fn new() -> Box {
    // Init components
    let tab = tab::new();

    // Init widget
    let tray = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(8)
        .build();

    // Compose childs
    tray.append(&menu::new()); // @TODO
    tray.append(tab.widget.as_ref());

    tray // @TODO struct
}
