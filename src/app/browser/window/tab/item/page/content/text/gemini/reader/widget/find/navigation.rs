mod back;
mod forward;

use super::MARGIN;
use gtk::{prelude::BoxExt, Box, Button, Orientation};

pub struct Navigation {
    pub back: Button,
    pub forward: Button,
    pub g_box: Box,
}

impl Navigation {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        // Init components
        let back = back::new();
        let forward = forward::new();

        // Init main container
        let g_box = Box::builder()
            .css_classes([
                "linked", // merge childs
            ])
            .margin_end(MARGIN)
            .orientation(Orientation::Horizontal)
            .build();

        g_box.append(&back);
        g_box.append(&forward);

        Self {
            back,
            forward,
            g_box,
        }
    }
}
