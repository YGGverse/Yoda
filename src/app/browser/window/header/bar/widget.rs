use adw::TabBar;
use gtk::{prelude::BoxExt, Box, MenuButton, Orientation, WindowControls};
use std::rc::Rc;

pub struct Widget {
    gobject: Box,
}

impl Widget {
    // Construct
    pub fn new_rc(control: &WindowControls, menu: &MenuButton, tab: &TabBar) -> Rc<Self> {
        let gobject = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .build();

        gobject.append(tab);
        gobject.append(menu);
        gobject.append(control);

        Rc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
