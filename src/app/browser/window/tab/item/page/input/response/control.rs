mod counter;
mod send;

use counter::Counter;
use send::Send;

use gtk::{gio::SimpleAction, prelude::BoxExt, Align, Box, Orientation};
use std::rc::Rc;

const SPACING: i32 = 8;

pub struct Control {
    pub counter: Rc<Counter>,
    pub send: Rc<Send>,
    pub g_box: Box,
}

impl Control {
    // Constructors

    /// Build new `Self`
    pub fn build(action_send: SimpleAction) -> Self {
        // Init components
        let counter = Rc::new(Counter::new());
        let send = Rc::new(Send::build(action_send));

        // Init main widget
        let g_box = Box::builder()
            .halign(Align::End)
            .orientation(Orientation::Horizontal)
            .spacing(SPACING)
            .build();

        g_box.append(&counter.label);
        g_box.append(&send.button);

        // Return activated struct
        Self {
            counter,
            send,
            g_box,
        }
    }

    // Actions
    pub fn update(&self, is_empty: bool, bytes_left: Option<isize>) {
        // Update children components
        self.counter.update(is_empty, bytes_left);
        self.send.update(match bytes_left {
            Some(left) => !is_empty && left >= 0,
            None => false,
        });
    }
}
