mod back;
mod forward;

use back::Back;
use forward::Forward;

use gtk::{prelude::BoxExt, Box, Orientation, TextIter};
use std::{cell::RefCell, rc::Rc};

const MARGIN: i32 = 6;

pub struct Navigation {
    pub back: Back,
    pub forward: Forward,
    pub g_box: Box,
    matches: Rc<RefCell<Vec<(TextIter, TextIter)>>>,
}

impl Navigation {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        // Init shared matches holder
        let matches = Rc::new(RefCell::new(Vec::new()));

        // Init components
        let back = Back::new();
        let forward = Forward::new();

        // Init main container
        let g_box = Box::builder()
            .css_classes([
                "linked", // merge childs
            ])
            .margin_end(MARGIN)
            .orientation(Orientation::Horizontal)
            .build();

        g_box.append(&back.button);
        g_box.append(&forward.button);

        Self {
            back,
            forward,
            g_box,
            matches,
        }
    }

    // Actions

    pub fn update(&self, matches: Vec<(TextIter, TextIter)>) {
        // Update self
        self.matches.replace(matches);

        // Update child components
        self.back.update(self.is_match());
        self.forward.update(self.is_match());
    }

    // pub fn back(&self) {}

    // pub fn forward(&self) {}

    // Getters

    pub fn is_match(&self) -> bool {
        !self.matches.borrow().is_empty()
    }
}
