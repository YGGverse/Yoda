use adw::TabBar;
use gtk::{prelude::BoxExt, Box, Button, MenuButton, Orientation, WindowControls};
use std::sync::Arc;

pub struct Widget {
    gobject: Box,
}

impl Widget {
    // Construct
    pub fn new_arc(
        control: &WindowControls,
        append: &Button,
        menu: &MenuButton,
        tab: &TabBar,
    ) -> Arc<Self> {
        let gobject = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .build();

        gobject.append(tab);
        gobject.append(append);
        gobject.append(menu);
        gobject.append(control);

        Arc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
