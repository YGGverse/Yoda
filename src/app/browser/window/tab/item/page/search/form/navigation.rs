mod back;
mod forward;

use back::Back;
use forward::Forward;

use super::Subject;
use gtk::{
    prelude::{BoxExt, TextBufferExt, TextViewExt},
    Box, Orientation, TextIter,
};
use std::{cell::RefCell, iter::Cycle, vec::IntoIter};

const MARGIN: i32 = 6;

pub struct Navigation {
    pub back: Back,
    pub forward: Forward,
    pub g_box: Box,
    iter: RefCell<Cycle<IntoIter<(TextIter, TextIter)>>>,
}

impl Navigation {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
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
            iter: RefCell::new(Vec::new().into_iter().cycle()),
        }
    }

    // Actions

    pub fn update(&self, matches: Vec<(TextIter, TextIter)>) {
        self.back.update(!matches.is_empty());
        self.forward.update(!matches.is_empty());
        let _ = self.iter.replace(matches.into_iter().cycle());
    }

    pub fn back(&self, subject: &Subject) -> Option<(TextIter, TextIter)> {
        let buffer = subject.text_view.buffer();

        buffer.remove_tag(
            &subject.tag.current,
            &buffer.start_iter(),
            &buffer.end_iter(),
        );

        match self.iter.borrow_mut().next() {
            Some((start, end)) => {
                buffer.apply_tag(&subject.tag.current, &start, &end);
                Some((start, end))
            }
            None => None,
        } // @TODO reverse
    }

    pub fn forward(&self, subject: &Subject) -> Option<(TextIter, TextIter)> {
        let buffer = subject.text_view.buffer();

        buffer.remove_tag(
            &subject.tag.current,
            &buffer.start_iter(),
            &buffer.end_iter(),
        );

        match self.iter.borrow_mut().next() {
            Some((start, end)) => {
                buffer.apply_tag(&subject.tag.current, &start, &end);
                Some((start, end))
            }
            None => None,
        }
    }
}
