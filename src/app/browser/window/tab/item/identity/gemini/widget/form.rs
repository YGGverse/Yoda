mod file;
pub mod list;
mod name;

use file::File;
use list::{item::value::Value, List};
use name::Name;

use super::Action;
use gtk::{prelude::BoxExt, Box, Orientation};
use std::rc::Rc;

pub struct Form {
    // pub action: Rc<Action>,
    pub file: Rc<File>,
    pub list: Rc<List>,
    pub name: Rc<Name>,
    pub gobject: Box,
}

impl Form {
    // Constructors

    /// Create new `Self`
    pub fn new(action: Rc<Action>) -> Self {
        // Init components
        let list = Rc::new(List::new());
        let name = Rc::new(Name::new(action.clone()));
        let file = Rc::new(File::new(action.clone()));

        // Init main container
        let gobject = Box::builder().orientation(Orientation::Vertical).build();

        gobject.append(&list.gobject);
        gobject.append(&name.gobject);
        gobject.append(&file.gobject);

        // Connect events
        list.on_select({
            let file = file.clone();
            let name = name.clone();
            let update = action.update.clone();
            move |key| {
                // Change name entry visibility
                name.show(match key {
                    Value::GENERATE_NEW_AUTH => true,
                    _ => false,
                });

                // Change name entry visibility
                file.show(match key {
                    Value::IMPORT_PEM => true,
                    _ => false,
                });

                // Update widget
                update.activate();
            }
        });

        // Return activated `Self`
        Self {
            // action,
            gobject,
            file,
            list,
            name,
        }
    }

    // Actions

    /// Validate form in current set
    pub fn is_valid(&self) -> bool {
        match self.list.selected() {
            Value::GENERATE_NEW_AUTH => self.name.is_valid(),
            _ => true,
        }
    }
}
