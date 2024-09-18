#[path = "main/tab.rs"] mod tab;

use gtk::Box;
use gtk::prelude::BoxExt;

pub fn new() -> Box
{
    let main = Box::builder().orientation(
        gtk::Orientation::Vertical
    ).build();

    main.append(
        &tab::new()
    );

    return main;
}