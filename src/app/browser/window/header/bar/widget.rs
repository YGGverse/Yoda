use adw::TabBar;
use gtk::{prelude::BoxExt, Box, MenuButton, Orientation, WindowControls};

pub struct Widget {
    pub gobject: Box,
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
}
