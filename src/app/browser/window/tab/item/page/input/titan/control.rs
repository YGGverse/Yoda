mod counter;
mod send;

use counter::Counter;
use gtk::{
    prelude::{BoxExt, WidgetExt},
    Align, Box, Button, Label, Orientation,
};
pub use send::Send;

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
    pub fn update(&self, chars_count: i32, bytes_total: usize) {
        // Update children components
        self.counter.update(chars_count, bytes_total);
        self.send.set_sensitive(bytes_total > 0);
    }
}
