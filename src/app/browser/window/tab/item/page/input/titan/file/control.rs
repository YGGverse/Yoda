mod counter;
mod options;
mod upload;

use super::Header;
use gtk::Box;
use std::{cell::Cell, rc::Rc};

pub trait Control {
    fn control(header: &Rc<Cell<Header>>) -> Self;
}

impl Control for Box {
    fn control(header: &Rc<Cell<Header>>) -> Self {
        use counter::Counter;
        use gtk::{Button, Label};
        use options::Options;
        use upload::Upload;

        // Init components
        let counter = Label::counter();
        let options = Button::options(header);
        let upload = Button::upload();

        // Init main widget
        {
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
        }
    }
}
