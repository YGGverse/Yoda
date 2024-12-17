mod back;
mod forward;
mod iter;

use back::Back;
use forward::Forward;
use iter::Iter;

use super::Subject;
use gtk::{
    prelude::{BoxExt, TextBufferExt, TextViewExt},
    Box, Orientation, TextIter,
};
use std::cell::RefCell;

const MARGIN: i32 = 6;

pub struct Navigation {
    pub back: Back,
    pub forward: Forward,
    pub g_box: Box,
    iter: RefCell<Option<Iter>>,
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
            iter: RefCell::new(None),
        }
    }

    // Actions

    pub fn update(&self, matches: Vec<(TextIter, TextIter)>) {
        self.back.update(!matches.is_empty());
        self.forward.update(!matches.is_empty());
        self.iter.replace(Some(Iter::new(matches)));
    }

    pub fn back(&self, subject: &Subject) -> Option<(TextIter, TextIter)> {
        let buffer = subject.text_view.buffer();

        buffer.remove_tag(
            &subject.tag.current,
            &buffer.start_iter(),
            &buffer.end_iter(),
        );

        match self.iter.borrow_mut().as_mut() {
            Some(iter) => match iter.back() {
                Some((start, end)) => {
                    buffer.apply_tag(&subject.tag.current, &start, &end);
                    Some((start, end))
                }
                None => iter.reset(),
            },
            None => todo!(), // unexpected
        }
    }

    pub fn forward(&self, subject: &Subject) -> Option<(TextIter, TextIter)> {
        let buffer = subject.text_view.buffer();

        buffer.remove_tag(
            &subject.tag.current,
            &buffer.start_iter(),
            &buffer.end_iter(),
        );

        match self.iter.borrow_mut().as_mut() {
            Some(iter) => match iter.forward() {
                Some((start, end)) => {
                    buffer.apply_tag(&subject.tag.current, &start, &end);
                    Some((start, end))
                }
                None => iter.reset(),
            },
            None => todo!(), // unexpected
        }
    }
}
