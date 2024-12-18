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

    /// Update widget state, including child components
    pub fn update(&self, matches: Vec<(TextIter, TextIter)>) {
        self.back.update(!matches.is_empty());
        self.forward.update(!matches.is_empty());
        self.model.replace(Model::new(matches));
    }

    /// Navigate back in matches, apply tags to buffer
    /// * return `start`/`end` iters to scroll up the widget
    /// * user should not activate this function on empty results
    ///   expected all actions / buttons deactivated in this case
    pub fn back(&self, subject: &Subject) -> (TextIter, TextIter) {
        let buffer = subject.text_view.buffer();

        buffer.remove_tag(
            &subject.tag.current,
            &buffer.start_iter(),
            &buffer.end_iter(),
        );

        let mut model = self.model.borrow_mut();

        let (start, end) = match model.back() {
            Some((start, end)) => (*start, *end),
            None => todo!(), // unexpected
        };

        if model.position().is_some() {
            buffer.apply_tag(&subject.tag.current, &start, &end);
        }

        (start, end)
    }

    /// Navigate forward in matches, apply tags to buffer
    /// * return `start`/`end` iters to scroll down the widget
    /// * user should not activate this function on empty results
    ///   expected all actions / buttons deactivated in this case
    pub fn forward(&self, subject: &Subject) -> (TextIter, TextIter) {
        let buffer = subject.text_view.buffer();

        buffer.remove_tag(
            &subject.tag.current,
            &buffer.start_iter(),
            &buffer.end_iter(),
        );

        let mut model = self.model.borrow_mut();

        let (start, end) = match model.next() {
            Some((start, end)) => (*start, *end),
            None => todo!(), // unexpected
        };

        if model.position().is_some() {
            buffer.apply_tag(&subject.tag.current, &start, &end);
        }

        (start, end)
    }

    // Getters

    pub fn position(&self) -> Option<usize> {
        self.model.borrow().position()
    }

    pub fn total(&self) -> usize {
        self.model.borrow().total()
    }
}
