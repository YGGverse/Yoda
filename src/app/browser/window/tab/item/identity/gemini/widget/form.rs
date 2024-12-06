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
    // pub action: Rc<Action>,
    pub drop: Rc<Drop>,
    pub exit: Rc<Exit>,
    pub file: Rc<File>,
    pub list: Rc<List>,
    pub name: Rc<Name>,
    pub save: Rc<Save>,
    pub g_box: Box,
}

impl Form {
    // Constructors

    /// Create new `Self`
    pub fn new(profile: Rc<Profile>, action: Rc<Action>, auth_url: &str) -> Self {
        // Init components
        let file = Rc::new(File::new(action.clone()));
        let list = Rc::new(List::new(profile.clone(), action.clone(), auth_url));
        let name = Rc::new(Name::new(action.clone()));
        let save = Rc::new(Save::new(profile.clone()));
        let drop = Rc::new(Drop::new(profile.clone(), action.clone(), list.clone()));
        let exit = Rc::new(Exit::new(profile.clone(), action.clone(), list.clone()));

        // Init main container
        let g_box = Box::builder().orientation(Orientation::Vertical).build();

        g_box.append(&list.dropdown);
        g_box.append(&name.entry);
        g_box.append(&file.button);
        g_box.append(&exit.button);
        g_box.append(&drop.button);
        g_box.append(&save.button);

        // Return activated `Self`
        Self {
            // action,
            drop,
            exit,
            file,
            list,
            name,
            save,
            g_box,
        }
    }

    // Actions

    /// Validate `Self` components match current selection
    pub fn is_applicable(&self) -> bool {
        match self.list.selected_item().value_enum() {
            Value::GeneratePem => self.name.is_valid(),
            Value::ImportPem => self.file.is_valid(),
            Value::ProfileIdentityGeminiId(_) => !self.list.selected_item().is_active(),
            _ => true,
        }
    }

    pub fn update(&self) {
        // Get selected item
        let item = self.list.selected_item();

        // Update name entry visibility
        self.name
            .update(matches!(item.value_enum(), Value::GeneratePem));

        // Update file choose button visibility
        self.file
            .update(matches!(item.value_enum(), Value::ImportPem));

        // Update ID-related components
        match item.value_enum() {
            Value::ProfileIdentityGeminiId(value) => {
                self.drop.update(Some(value));
                self.exit.update(Some(value));
                self.save.update(Some(value));
            }
            _ => {
                self.drop.update(None);
                self.exit.update(None);
                self.save.update(None);
            }
        }
    }
}
