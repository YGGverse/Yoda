mod back;
mod forward;

use back::Back;
use forward::Forward;

use gtk::{
    prelude::{BoxExt, TextBufferExt},
    Box, Orientation, TextBuffer, TextIter, TextTag,
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

const MARGIN: i32 = 6;

pub struct Navigation {
    pub back: Back,
    pub forward: Forward,
    pub g_box: Box,
    index: Rc<Cell<usize>>,
    matches: Rc<RefCell<Vec<(TextIter, TextIter)>>>,
    text_buffer: TextBuffer,
    current_tag: TextTag,
}

impl Navigation {
    // Constructors

    /// Create new `Self`
    pub fn new(text_buffer: TextBuffer, current_tag: TextTag) -> Self {
        // Init shared matches holder
        let index = Rc::new(Cell::new(0));
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
            index,
            matches,
            text_buffer,
            current_tag,
        }
    }

    // Actions

    pub fn update(&self, matches: Vec<(TextIter, TextIter)>) {
        // Update self
        self.matches.replace(matches);
        self.index.replace(0); // reset

        // Update child components
        self.back.update(self.is_match());
        self.forward.update(self.is_match());
    }

    pub fn back(&self) -> Option<(TextIter, TextIter)> {
        self.text_buffer.remove_tag(
            &self.current_tag,
            &self.text_buffer.start_iter(),
            &self.text_buffer.end_iter(),
        );

        let index = self.index.take();
        match self.matches.borrow().get(back(index)) {
            Some((start, end)) => {
                self.text_buffer.apply_tag(&self.current_tag, &start, &end);
                self.index.replace(if index == 0 {
                    len_to_index(self.matches.borrow().len())
                } else {
                    index
                });
                Some((*start, *end))
            }
            None => {
                self.index
                    .replace(len_to_index(self.matches.borrow().len())); // go last
                None
            }
        }
    }

    pub fn forward(&self) -> Option<(TextIter, TextIter)> {
        self.text_buffer.remove_tag(
            &self.current_tag,
            &self.text_buffer.start_iter(),
            &self.text_buffer.end_iter(),
        );

        let index = self.index.take();
        let next = forward(index);
        match self.matches.borrow().get(next) {
            Some((start, end)) => {
                self.text_buffer.apply_tag(&self.current_tag, &start, &end);
                self.index.replace(next);
                Some((*start, *end))
            }
            None => {
                self.index.replace(0);
                None
            }
        }
    }

    // Getters

    pub fn is_match(&self) -> bool {
        !self.matches.borrow().is_empty()
    }
}

fn back(index: usize) -> usize {
    index - 1
}

fn forward(index: usize) -> usize {
    index + 1
}

fn len_to_index(len: usize) -> usize {
    len - 1
}
