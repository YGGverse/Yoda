use adw::{TabView, ToolbarView};
use gtk::{prelude::BoxExt, Box, Orientation};

pub struct Widget {
    pub g_box: Box,
}

impl Widget {
    // Constructors

    /// Build new `Self`
    pub fn build(header: &ToolbarView, tab: &TabView) -> Self {
        let g_box = Box::builder().orientation(Orientation::Vertical).build();

        g_box.append(header);
        g_box.append(tab);

        Self { g_box }
    }
}
