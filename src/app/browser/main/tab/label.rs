#[path = "label/pin.rs"] mod pin;
#[path = "label/title.rs"] mod title;

use gtk::Box;
use gtk::prelude::BoxExt;

pub fn new() -> Box
{
    let label = Box::builder()

        // Tuneup
        .orientation(
            gtk::Orientation::Horizontal
        )

        .build();

        // Components
        label.append(
            &pin::new(
                false
            )
        );

        label.append(
            &title::new()
        );

    return label;
}