mod back;
mod forward;

use std::{cell::Cell, rc::Rc};

use super::MARGIN;
use gtk::{prelude::BoxExt, Box, Button, Orientation, TextIter};

pub struct Navigation {
    pub back: Button,
    pub forward: Button,
    pub g_box: Box,
    pub matches: Rc<Cell<Vec<(TextIter, TextIter)>>>,
}

impl Navigation {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        let matches = Rc::new(Cell::new(Vec::new()));

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
            matches,
        }
    }

    pub fn update(&self, matches: Vec<(TextIter, TextIter)>) {
        self.matches.replace(matches);
    }
}
