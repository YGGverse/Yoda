mod drop;
mod file;
pub mod list;
mod name;
mod save;

use drop::Drop;
use file::File;
use list::{item::value::Value, List};
use name::Name;
use save::Save;

use super::Action;
use crate::profile::Profile;
use gtk::{prelude::BoxExt, Box, Orientation};
use std::rc::Rc;

pub struct Form {
    // pub action: Rc<Action>,
    // pub drop: Rc<Drop>,
    pub file: Rc<File>,
    pub list: Rc<List>,
    pub name: Rc<Name>,
    // pub save: Rc<Save>,
    pub gobject: Box,
}

impl Form {
    // Constructors

    /// Create new `Self`
    pub fn new(profile: Rc<Profile>, action: Rc<Action>) -> Self {
        // Init components
        let file = Rc::new(File::new(action.clone()));
        let list = Rc::new(List::new());
        let name = Rc::new(Name::new(action.clone()));
        let save = Rc::new(Save::new(profile.clone()));
        let drop = Rc::new(Drop::new(profile.clone(), action.clone(), list.clone()));

        // Init main container
        let gobject = Box::builder().orientation(Orientation::Vertical).build();

        gobject.append(&list.gobject);
        gobject.append(&name.gobject);
        gobject.append(&file.gobject);
        gobject.append(&drop.gobject);
        gobject.append(&save.gobject);

        // Connect events
        list.on_select({
            let drop = drop.clone();
            let file = file.clone();
            let name = name.clone();
            let save = save.clone();
            let update = action.update.clone();
            move |item| {
                // Change name entry visibility
                name.update(matches!(item, Value::GenerateNewAuth));

                // Change file choose button visibility
                file.update(matches!(item, Value::ImportPem));

                // Change other components visibility by update it holder value
                match item {
                    Value::ProfileIdentityGeminiId(value) => {
                        drop.update(Some(value));
                        save.update(Some(value));
                    }
                    _ => {
                        drop.update(None);
                        save.update(None);
                    }
                }

                // Update widget
                update.activate();
            }
        });

        // Return activated `Self`
        Self {
            // action,
            // drop,
            file,
            list,
            name,
            // save,
            gobject,
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
