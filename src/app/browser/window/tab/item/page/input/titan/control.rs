mod counter;
mod send;

use counter::Counter;
use gtk::gio::SimpleAction;
use gtk::{prelude::BoxExt, Align, Box, Orientation};
use send::Send;
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
    pub fn update(&self, bytes_total: Option<usize>) {
        // Update children components
        self.counter.update(bytes_total);
        self.send.update(match bytes_total {
            Some(total) => total > 0,
            None => false,
        });
    }
}
