pub mod drop;
pub mod list;
pub mod query;

use crate::Profile;
use drop::Drop;
use gtk::{Box, Button, Entry, Orientation, prelude::BoxExt};
use list::List;
pub use query::Query;
use std::rc::Rc;

pub struct Form {
    pub drop: Button,
    pub list: Rc<List>,
    pub query: Entry,
    pub g_box: Box,
}

impl Form {
    // Constructors

    /// Create new `Self`
    pub fn build(profile: &Rc<Profile>) -> Self {
        // Init components
        let list = Rc::new(List::build(profile));
        let query = Entry::query();
        let drop = Button::drop(profile, &list);

        // Init main container
        let g_box = Box::builder().orientation(Orientation::Vertical).build();

        g_box.append(&list.dropdown);
        g_box.append(&query);
        g_box.append(&drop);

        Self {
            drop,
            list,
            query,
            g_box,
        }
    }
}
