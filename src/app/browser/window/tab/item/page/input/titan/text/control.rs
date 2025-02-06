mod counter;
mod options;
mod upload;

use super::Header;
use counter::Counter;
use gtk::{
    prelude::{BoxExt, WidgetExt},
    Align, Box, Button, Label, Orientation,
};
use options::Options;
use std::{cell::Cell, rc::Rc};
pub use upload::Upload;

const SPACING: i32 = 8;

pub struct Control {
    pub counter: Label,
    pub upload: Button,
    pub g_box: Box,
}

impl Control {
    // Constructors

    /// Build new `Self`
    pub fn build(header: &Rc<Cell<Header>>) -> Self {
        // Init components
        let counter = Label::counter();
        let options = Button::options(header);
        let upload = Button::upload();

        // Init main widget
        let g_box = Box::builder()
            .halign(Align::End)
            .orientation(Orientation::Horizontal)
            .spacing(SPACING)
            .build();

        g_box.append(&counter);
        g_box.append(&options);
        g_box.append(&upload);

        // Return activated struct
        Self {
            counter,
            upload,
            g_box,
        }
    }

    // Actions
    pub fn update(&self, chars_count: i32, bytes_total: usize) {
        // Update children components
        self.counter.update(chars_count, bytes_total);
        self.upload.set_sensitive(bytes_total > 0);
    }
}
