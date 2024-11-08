use gtk::{prelude::BoxExt, Box, Label, Orientation, TextView};
use std::rc::Rc;

const MARGIN: i32 = 6;
const SPACING: i32 = 8;

pub struct Widget {
    gobject: Box,
}

impl Widget {
    // Construct
    pub fn new_rc(title: &Label, response: &TextView, control: &Box) -> Rc<Self> {
        let gobject = Box::builder()
            .margin_bottom(MARGIN)
            .margin_end(MARGIN)
            .margin_start(MARGIN)
            .margin_top(MARGIN)
            .spacing(SPACING)
            .orientation(Orientation::Vertical)
            .build();

        gobject.append(title);
        gobject.append(response);
        gobject.append(control);

        Rc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
