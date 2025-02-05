mod counter;
mod upload;

use counter::Counter;
use gtk::{
    prelude::{BoxExt, WidgetExt},
    Align, Box, Button, Label, Orientation,
};
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
    pub fn build() -> Self {
        // Init components
        let counter = Label::counter();
        let upload = Button::upload();

        // Init main widget
        let g_box = Box::builder()
            .halign(Align::End)
            .orientation(Orientation::Horizontal)
            .spacing(SPACING)
            .build();

        g_box.append(&counter);
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
