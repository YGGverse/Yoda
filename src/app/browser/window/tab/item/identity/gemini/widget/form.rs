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
            move |item| {
                // Change name entry visibility
                name.show(match item {
                    Value::GenerateNewAuth => true,
                    _ => false,
                });

                // Change file choose button visibility
                file.show(match item {
                    Value::ImportPem => true,
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

    /// Validate `Self` components match current selection
    pub fn is_valid(&self) -> bool {
        match self.list.selected() {
            Value::GenerateNewAuth => self.name.is_valid(),
            Value::ImportPem => self.file.is_valid(),
            _ => true,
        }
    }
}
