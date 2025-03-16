mod drop;
mod exit;
mod file;
pub mod list;
mod name;
mod save;

use drop::Drop;
use exit::Exit;
use file::File;
use list::{List, item::value::Value};
use name::Name;
use save::Save;

use super::WidgetAction;
use crate::Profile;
use gtk::{
    Box, Button, Entry, Orientation,
    glib::Uri,
    prelude::{BoxExt, WidgetExt},
};
use std::{rc::Rc, sync::Arc};

pub struct Form {
    // pub action_widget: Rc<Action>,
    pub drop: Rc<Drop>,
    pub exit: Button,
    pub file: Rc<File>,
    pub list: Rc<List>,
    pub name: Entry,
    pub save: Rc<Save>,
    pub g_box: Box,
    profile: Arc<Profile>,
}

impl Form {
    // Constructors

    /// Create new `Self`
    pub fn build(widget_action: &Rc<WidgetAction>, profile: &Arc<Profile>, request: &Uri) -> Self {
        // Init components
        let list = Rc::new(List::build(widget_action, profile, request));
        let file = Rc::new(File::build(widget_action));
        let name = Entry::name(widget_action);
        let save = Rc::new(Save::build(profile, &list));
        let drop = Rc::new(Drop::build(profile, &list));
        let exit = Button::exit(widget_action, profile, &list, request);

        // Init main container
        let g_box = Box::builder().orientation(Orientation::Vertical).build();

        g_box.append(&list.dropdown);
        g_box.append(&name);
        g_box.append(&file.button);
        g_box.append(&exit);
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
            profile: profile.clone(),
        }
    }

    // Actions

    /// Get `Apply` button sensitivity to disable when it does not change anything
    pub fn is_applicable(&self) -> bool {
        match self.list.selected().value_enum() {
            Value::GeneratePem => self.name.is_valid(),
            Value::ImportPem => self.file.is_valid(),
            _ => !self.list.selected().is_active(),
        }
    }

    pub fn update(&self) {
        // Get shared selected item value
        let value = self.list.selected().value_enum();

        // Begin children components update
        self.name.update(matches!(value, Value::GeneratePem));
        self.file.update(matches!(value, Value::ImportPem));

        match value {
            Value::ProfileIdentityId(profile_identity_id) => {
                self.drop.update(true);
                self.exit.set_visible(true);
                self.exit
                    .set_sensitive(self.profile.identity.auth.total(profile_identity_id) > 0);
                self.save.update(true);
            }
            _ => {
                self.drop.update(false);
                self.exit.set_visible(false);
                self.exit.set_sensitive(false);
                self.save.update(false);
            }
        }
    }
}
