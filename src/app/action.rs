// This helper created as the attempt to drop static names usage
// and replace them with objects (to follow encapsulation for children mods)
// @TODO find alternative implementation, if exist for GTK 4

use gtk::{
    gio::SimpleAction,
    glib::{gformat, uuid_string_random, GString, VariantTy},
    prelude::ActionExt,
};

pub struct Action {
    group: GString,
    simple: SimpleAction,
}

impl Action {
    // Construct
    pub fn new(group: &str, is_enabled: bool, parameter_type: Option<&VariantTy>) -> Self {
        // Create random action name as no static values should be in use
        let simple = SimpleAction::new(&uuid_string_random(), parameter_type);
        simple.set_enabled(is_enabled);

        // Assign action to the group
        let group = GString::from(group);

        // Return new Action
        Self { group, simple }
    }

    // Getters
    pub fn detailed_name(&self) -> GString {
        gformat!("{}.{}", self.group, self.simple.name()) // @TODO find the way to ident parent group
                                                          // from SimpleAction object
    }

    // App mods work with simple and system-wide data types, let them take it
    pub fn simple(&self) -> SimpleAction {
        self.simple.clone()
    }
}
