mod back;
mod forward;

use back::Back;
use forward::Forward;
use gtk::prelude::BoxExt;
use gtk::{Box, Orientation};

pub struct History {
    widget: Box,
}

impl History {
    // Construct
    pub fn new() -> History {
        // init components
        let back = Back::new();
        let forward = Forward::new();

        // Init widget
        let widget = Box::builder()
            .orientation(Orientation::Horizontal)
            .css_classes([
                "linked", // merge childs
            ])
            .build();

        widget.append(back.widget());
        widget.append(forward.widget());

        Self { widget }
    }

    // Getters
    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
