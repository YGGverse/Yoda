mod counter;
mod options;
mod upload;

use super::Header;
use counter::Counter;
use gtk::{Box, Button, Label};
use options::Options;
use std::{cell::Cell, rc::Rc};
use upload::Upload;

pub struct Control {
    counter: Label,
    options: Button,
    upload: Button,
    pub g_box: Box,
}

impl Control {
    pub fn build(header: &Rc<Cell<Header>>) -> Self {
        // Init components
        let counter = Label::counter();
        let options = Button::options(header);
        let upload = Button::upload();

        // Init main widget
        let g_box = {
            use gtk::{prelude::BoxExt, Align, Orientation};
            let g_box = Box::builder()
                .halign(Align::End)
                .orientation(Orientation::Horizontal)
                .spacing(8)
                .build();

            g_box.append(&counter);
            g_box.append(&options);
            g_box.append(&upload);
            g_box
        };

        Self {
            counter,
            options,
            upload,
            g_box,
        }
    }

    pub fn update(&self, bytes_total: Option<usize>) {
        use gtk::prelude::WidgetExt;

        self.counter.update(bytes_total);

        let is_some = bytes_total.is_some();
        self.options.set_sensitive(is_some);
        self.upload.set_sensitive(is_some);
    }
}
