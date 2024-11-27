use adw::{TabView, ToolbarView};
use gtk::{prelude::BoxExt, Box, Orientation};

pub struct Widget {
    pub gobject: Box,
}

impl Widget {
    // Construct
    pub fn new(header: &ToolbarView, tab: &TabView) -> Self {
        let gobject = Box::builder().orientation(Orientation::Vertical).build();
        gobject.append(header);
        gobject.append(tab);

        Self { gobject }
    }
}
