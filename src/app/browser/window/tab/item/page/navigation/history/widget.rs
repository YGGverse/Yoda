use gtk::{
    prelude::{BoxExt, IsA},
    Box, Orientation,
};
use std::rc::Rc;

pub struct Widget {
    gobject: Box,
}

impl Widget {
    // Construct
    pub fn new_rc(back: &impl IsA<gtk::Widget>, forward: &impl IsA<gtk::Widget>) -> Rc<Self> {
        // Init widget
        let gobject = Box::builder()
            .orientation(Orientation::Horizontal)
            .css_classes([
                "linked", // merge childs
            ])
            .build();

        // Compose childs
        gobject.append(back);
        gobject.append(forward);

        // Return activated struct
        Rc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
