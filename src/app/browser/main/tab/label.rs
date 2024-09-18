#[path = "label/pin.rs"] mod pin;
#[path = "label/title.rs"] mod title;

use gtk::Box;
use gtk::prelude::BoxExt;

pub fn new() -> Box
{
    let label = Box::builder().orientation(
        gtk::Orientation::Horizontal
    ).build();

    label.append(
        &pin::new()
    );

    label.append(
        &title::new()
    );

    return label;
}