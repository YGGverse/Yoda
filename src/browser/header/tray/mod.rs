mod menu;
mod tab;

use gtk::prelude::BoxExt;
use gtk::Box;

pub fn new() -> Box {
    let tray = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(8)
        .build();

    // Compose childs
    tray.append(&menu::new());
    tray.append(&tab::new());

    tray
}
