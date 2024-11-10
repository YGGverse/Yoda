use adw::TabBar;
use gtk::{prelude::BoxExt, Box, MenuButton, Orientation, WindowControls};

pub struct Widget {
    gobject: Box,
}

impl Widget {
    // Construct
    pub fn new(control: &WindowControls, menu: &MenuButton, tab: &TabBar) -> Self {
        let gobject = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .build();

        gobject.append(tab);
        gobject.append(menu);
        gobject.append(control);

        Self { gobject }
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
