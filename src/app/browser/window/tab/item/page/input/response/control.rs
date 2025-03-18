mod counter;
mod send;

use counter::Counter;
use gtk::{Align, Box, Button, Label, Orientation, prelude::BoxExt};
use send::Send;

const SPACING: i32 = 8;

pub struct Control {
    pub counter: Label,
    pub send: Button,
    pub g_box: Box,
}

impl Control {
    // Constructors

    /// Build new `Self`
    pub fn build() -> Self {
        // Init components
        let counter = Label::counter();
        let send = Button::send();

        // Init main widget
        let g_box = Box::builder()
            .halign(Align::End)
            .orientation(Orientation::Horizontal)
            .spacing(SPACING)
            .build();

        g_box.append(&counter);
        g_box.append(&send);

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
