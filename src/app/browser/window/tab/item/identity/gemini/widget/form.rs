mod list;
mod name;

use list::List;
use name::Name;

use gtk::{
    prelude::{BoxExt, WidgetExt},
    Box, Orientation,
};
use std::rc::Rc;

pub struct Form {
    pub gobject: Box,
    pub list: Rc<List>,
    pub name: Rc<Name>,
}

impl Form {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        // Init components
        let list = Rc::new(List::new());
        let name = Rc::new(Name::new());

        // Init main container
        let gobject = Box::builder().orientation(Orientation::Vertical).build();

        gobject.append(&list.gobject);
        gobject.append(&name.gobject);

        // Connect events
        list.on_select({
            let name = name.clone();
            // Show name entry on new identity option selected
            move |key| name.gobject.set_visible(key.is_none())
        });

        // Return activated `Self`
        Self {
            gobject,
            list,
            name,
        }
    }
}
