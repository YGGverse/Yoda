#[path = "history/back.rs"] mod back;
#[path = "history/forward.rs"] mod forward;

use gtk::Box;
use gtk::prelude::BoxExt;

pub fn new() -> Box
{
    let history = Box::builder()

        // Tuneup
        .orientation(
            gtk::Orientation::Horizontal
        )

        .css_classes(
            [
                "linked" // merge childs
            ]
        )

        .build();

    // Compose childs
    history.append(
        &back::new()
    );

    history.append(
        &forward::new()
    );

    return history;
}