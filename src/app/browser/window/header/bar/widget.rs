use adw::TabBar;
use gtk::{prelude::BoxExt, Box, MenuButton, Orientation, WindowControls};

pub struct Widget {
    pub g_box: Box,
}

impl Widget {
    // Construct
    pub fn new(control: &WindowControls, menu: &MenuButton, tab: &TabBar) -> Self {
        let g_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .build();

        g_box.append(tab);
        g_box.append(menu);
        g_box.append(control);

        Self { g_box }
    }
}
