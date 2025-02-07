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
        let g_box = {
            const MARGIN: i32 = 8;
            Box::builder()
                .halign(Align::End)
                .margin_bottom(MARGIN)
                .orientation(Orientation::Horizontal)
                .spacing(MARGIN)
                .build()
        };

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
    pub fn update(&self, bytes_total: Option<usize>, chars_count: Option<i32>) {
        // Update children components
        self.counter.update(bytes_total, chars_count);
        self.upload
            .set_sensitive(bytes_total.is_some_and(|this| this > 0));
    }
}
