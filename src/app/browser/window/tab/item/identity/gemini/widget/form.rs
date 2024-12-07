mod drop;
mod exit;
mod file;
pub mod list;
mod name;
mod save;

use drop::Drop;
use exit::Exit;
use file::File;
use list::{item::value::Value, List};
use name::Name;
use save::Save;

use super::Action;
use crate::profile::Profile;
use gtk::{prelude::BoxExt, Box, Orientation};
use std::rc::Rc;

pub struct Form {
    // pub widget_action: Rc<Action>,
    // pub drop: Rc<Drop>,
    // pub exit: Rc<Exit>,
    pub file: Rc<File>,
    pub list: Rc<List>,
    pub name: Rc<Name>,
    // pub save: Rc<Save>,
    pub g_box: Box,
}

impl Form {
    // Constructors

    /// Create new `Self`
    pub fn new(profile: Rc<Profile>, widget_action: Rc<Action>, auth_url: &str) -> Self {
        // Init components
        let list = Rc::new(List::new(profile.clone(), auth_url));
        let file = Rc::new(File::new(widget_action.clone()));
        let name = Rc::new(Name::new(widget_action.clone()));
        let save = Rc::new(Save::new(profile.clone(), list.clone()));
        let drop = Rc::new(Drop::new(profile.clone(), list.clone()));
        let exit = Rc::new(Exit::new(profile.clone(), list.clone()));

        // Init main container
        let g_box = Box::builder().orientation(Orientation::Vertical).build();

        g_box.append(&list.dropdown);
        g_box.append(&name.entry);
        g_box.append(&file.button);
        g_box.append(&exit.button);
        g_box.append(&drop.button);
        g_box.append(&save.button);

        // Connect events
        list.dropdown.connect_selected_notify({
            let list = list.clone();
            let name = name.clone();
            let file = file.clone();
            // let drop = drop.clone();
            // let exit = exit.clone();
            // let save = save.clone();
            move |_| {
                // Get selected item
                let item = list.selected();

                // Update name entry visibility
                name.set_visible(matches!(item.value_enum(), Value::GeneratePem));

                // Update file choose button visibility
                file.set_visible(matches!(item.value_enum(), Value::ImportPem));

                // Update ID-related components
                match item.value_enum() {
                    Value::ProfileIdentityGeminiId(_) => {
                        drop.set_visible(true);
                        exit.set_visible(true);
                        save.set_visible(true);
                    }
                    _ => {
                        drop.set_visible(false);
                        exit.set_visible(false);
                        save.set_visible(false);
                    }
                }
            }
        });

        // Return activated `Self`
        Self {
            // widget_action,
            // drop,
            // exit,
            file,
            list,
            name,
            // save,
            g_box,
        }
    }

    // Actions

    /// Validate `Self` components match current selection
    pub fn is_applicable(&self) -> bool {
        match self.list.selected().value_enum() {
            Value::GeneratePem => self.name.is_valid(),
            Value::ImportPem => self.file.is_valid(),
            Value::ProfileIdentityGeminiId(_) => !self.list.selected().is_active(),
            _ => true,
        }
    }
}
