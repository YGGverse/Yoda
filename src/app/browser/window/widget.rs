use adw::{TabView, ToolbarView};
use gtk::{prelude::BoxExt, Box, Orientation};

pub struct Widget {
    gobject: Box,
}

impl Widget {
    // Construct
    pub fn new(header: &ToolbarView, tab: &TabView) -> Self {
        let gobject = Box::builder().orientation(Orientation::Vertical).build();
        gobject.append(header);
        gobject.append(tab);

        Self { gobject }
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
