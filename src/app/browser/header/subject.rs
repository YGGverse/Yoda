#[path = "subject/title.rs"] mod title;
#[path = "subject/description.rs"] mod description;

use gtk::Box;
use gtk::prelude::BoxExt;

pub fn new() -> Box
{
    let subject = Box::builder()

        // Tuneup
        .orientation(
            gtk::Orientation::Vertical
        )

        .valign(
            gtk::Align::Center
        )

        .build();

        // Compose childs
        subject.append(
            &title::new()
        );

        subject.append(
            &description::new()
        );

    // Done
    subject
}