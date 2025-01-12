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

impl Default for Navigation {
    fn default() -> Self {
        Self::new()
    }
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

    /// Update navigation model
    pub fn renew(&self, matches: Vec<(TextIter, TextIter)>) {
        self.model.replace(Model::new(matches));
        self.update();
    }

    /// Update widget including child components
    pub fn update(&self) {
        let model = self.model.borrow();
        self.back.update(model.is_back());
        self.forward.update(model.is_next());
    }

    /// Navigate back in matches, apply tags to the buffer
    /// * return `start`/`end` iters to scroll up the widget
    pub fn back(&self, subject: &Subject) -> Option<(TextIter, TextIter)> {
        let buffer = subject.text_view.buffer();

        buffer.remove_tag(
            &subject.tag.current,
            &buffer.start_iter(),
            &buffer.end_iter(),
        );

        let mut model = self.model.borrow_mut();

        match model.back().map(|(start, end)| (*start, *end)) {
            Some((start, end)) => {
                if model.position().is_some() {
                    buffer.apply_tag(&subject.tag.current, &start, &end);
                }
                Some((start, end))
            }
            None => None,
        }
    }

    /// Navigate forward in matches, apply tags to the buffer
    /// * return `start`/`end` iters to scroll down the widget
    pub fn forward(&self, subject: &Subject) -> Option<(TextIter, TextIter)> {
        let buffer = subject.text_view.buffer();

        buffer.remove_tag(
            &subject.tag.current,
            &buffer.start_iter(),
            &buffer.end_iter(),
        );

        let mut model = self.model.borrow_mut();

        match model.next().map(|(start, end)| (*start, *end)) {
            Some((start, end)) => {
                if model.position().is_some() {
                    buffer.apply_tag(&subject.tag.current, &start, &end);
                }
                Some((start, end))
            }
            None => None,
        }
    }

    // Getters

    pub fn position(&self) -> Option<usize> {
        self.model.borrow().position()
    }

    pub fn total(&self) -> usize {
        self.model.borrow().total()
    }
}
