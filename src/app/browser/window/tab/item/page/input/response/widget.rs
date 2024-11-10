use gtk::{prelude::BoxExt, Box, Label, Orientation, TextView};

const MARGIN: i32 = 6;
const SPACING: i32 = 8;

pub struct Widget {
    gobject: Box,
}

impl Widget {
    // Construct
    pub fn new(title: &Label, response: &TextView, control: &Box) -> Self {
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

        Self { gobject }
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
