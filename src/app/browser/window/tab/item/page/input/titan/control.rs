mod counter;
mod options;
mod upload;

use counter::Counter;
use gtk::{
    Align, Box, Button, Label, Orientation,
    prelude::{BoxExt, WidgetExt},
};
use options::Options;
pub use upload::Upload;

pub struct Control {
    pub counter: Label,
    pub options: Button,
    pub upload: Button,
    pub g_box: Box,
}

impl Control {
    // Constructors

    /// Build new `Self`
    pub fn build() -> Self {
        // Init components
        let counter = Label::counter();
        let options = Button::options();
        let upload = Button::upload();

        // Init main widget
        let g_box = {
            const MARGIN: i32 = 8;
            Box::builder()
                .halign(Align::End)
                .margin_bottom(MARGIN / 2)
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
            options,
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
