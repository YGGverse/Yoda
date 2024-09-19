#[path = "tray/menu.rs"] mod menu;
#[path = "tray/tab.rs"] mod tab;

use gtk::Box;
use gtk::prelude::BoxExt;

pub fn new() -> Box
{
    let tray = Box::builder()

        // Tuneup
        .orientation(
            gtk::Orientation::Horizontal
        )

        .spacing(8)

        .build();

    // Compose childs
    tray.append(
        &menu::new()
    );

    tray.append(
        &tab::new()
    );

    return tray;
}