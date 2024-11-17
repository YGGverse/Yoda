mod list;
mod name;

use list::List;
use name::Name;

use gtk::{
    prelude::{BoxExt, WidgetExt},
    Box, Orientation,
};

pub struct Form {
    gobject: Box,
}

impl Form {
    // Constructors

    /// Create new `Self`
    pub fn new(items: Vec<(Option<i64>, String, bool)>) -> Self {
        // Init components
        let list = List::new();
        let name = Name::new();

        // Init main container
        let gobject = Box::builder().orientation(Orientation::Vertical).build();

        gobject.append(list.gobject());
        gobject.append(name.gobject());

        // Return activated `Self`
        Self { gobject }
    }

    // Getters

    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
