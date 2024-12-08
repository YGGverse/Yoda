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

use super::WidgetAction;
use crate::{
    app::browser::{action::Action as BrowserAction, window::action::Action as WindowAction},
    Profile,
};
use gtk::{glib::Uri, prelude::BoxExt, Box, Orientation};
use std::rc::Rc;

pub struct Form {
    // pub action_widget: Rc<Action>,
    pub drop: Rc<Drop>,
    pub exit: Rc<Exit>,
    pub file: Rc<File>,
    pub list: Rc<List>,
    pub name: Rc<Name>,
    pub save: Rc<Save>,
    pub g_box: Box,
    auth_uri: Uri,
    profile: Rc<Profile>,
}

impl Form {
    // Constructors

    /// Create new `Self`
    pub fn new(
        action: (Rc<BrowserAction>, Rc<WindowAction>, Rc<WidgetAction>),
        profile: Rc<Profile>,
        auth_uri: Uri,
    ) -> Self {
        // Init components
        let list = Rc::new(List::new(
            action.2.clone(),
            profile.clone(),
            auth_uri.clone(),
        ));
        let file = Rc::new(File::new(action.2.clone()));
        let name = Rc::new(Name::new(action.2.clone()));
        let save = Rc::new(Save::new(profile.clone(), list.clone()));
        let drop = Rc::new(Drop::new(profile.clone(), list.clone()));
        let exit = Rc::new(Exit::new(
            action.0.clone(),
            profile.clone(),
            list.clone(),
            auth_uri.clone(),
        ));

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
            // action_widget,
            drop,
            exit,
            file,
            list,
            name,
            save,
            g_box,
            auth_uri,
            profile,
        }
    }

    // Actions

    /// Get `Apply` button sensitivity to disable when it does not change anything
    pub fn is_applicable(&self) -> bool {
        match self.list.selected().value_enum() {
            Value::GeneratePem => self.name.is_valid(),
            Value::ImportPem => self.file.is_valid(),
            Value::ProfileIdentityGeminiId(_) => !self.list.selected().is_active(),
            _ => true,
        }
    }

    pub fn update(&self) {
        // Get shared selected item value
        let value = self.list.selected().value_enum();

        // Begin children components update
        self.name.update(matches!(value, Value::GeneratePem));
        self.file.update(matches!(value, Value::ImportPem));

        match value {
            Value::ProfileIdentityGeminiId(profile_identity_gemini_id) => {
                self.drop.update(true);
                self.exit.update(
                    true,
                    self.profile
                        .identity
                        .gemini
                        .auth
                        .memory
                        .match_scope(&self.auth_uri.to_string())
                        .is_some_and(|auth| {
                            auth.profile_identity_gemini_id == profile_identity_gemini_id
                        }),
                );
                self.save.update(true);
            }
            _ => {
                self.drop.update(false);
                self.exit.update(false, false);
                self.save.update(false);
            }
        }
    }
}
