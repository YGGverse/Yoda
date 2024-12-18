mod back;
mod forward;
mod model;

use back::Back;
use forward::Forward;
use model::Model;

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
    model: RefCell<Model<(TextIter, TextIter)>>,
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
            model: RefCell::new(Model::new(Vec::new())), // @TODO option?
        }
    }

    // Actions

    pub fn update(&self, matches: Vec<(TextIter, TextIter)>) {
        self.back.update(!matches.is_empty());
        self.forward.update(!matches.is_empty());
        self.model.replace(Model::new(matches));
    }

    pub fn back(&self, subject: &Subject) -> Option<(TextIter, TextIter)> {
        let buffer = subject.text_view.buffer();

        buffer.remove_tag(
            &subject.tag.current,
            &buffer.start_iter(),
            &buffer.end_iter(),
        );

        match self.model.borrow_mut().back() {
            Some((start, end)) => {
                buffer.apply_tag(&subject.tag.current, start, end);
                Some((*start, *end))
            }
            None => None,
        }
    }

    pub fn forward(&self, subject: &Subject) -> Option<(TextIter, TextIter)> {
        let buffer = subject.text_view.buffer();

        buffer.remove_tag(
            &subject.tag.current,
            &buffer.start_iter(),
            &buffer.end_iter(),
        );

        match self.model.borrow_mut().next() {
            Some((start, end)) => {
                buffer.apply_tag(&subject.tag.current, start, end);
                Some((*start, *end))
            }
            None => None,
        }
    }

    pub fn position(&self) -> usize {
        self.model.borrow().position()
    }

    pub fn total(&self) -> usize {
        self.model.borrow().total()
    }
}
