use gtk::{gio::SimpleAction, glib::uuid_string_random, prelude::StaticVariantType};

pub struct Action {
    about: SimpleAction,
    debug: SimpleAction,
    profile: SimpleAction,
    quit: SimpleAction,
    update: SimpleAction,
}

impl Action {
    // Constructors

    pub fn new() -> Self {
        Self {
            about: SimpleAction::new(&uuid_string_random(), None),
            debug: SimpleAction::new(&uuid_string_random(), None),
            profile: SimpleAction::new(&uuid_string_random(), None),
            quit: SimpleAction::new(&uuid_string_random(), None),
            update: SimpleAction::new(&uuid_string_random(), Some(&String::static_variant_type())),
        }
    }

    // Getters

    pub fn about(&self) -> &SimpleAction {
        &self.about
    }

    pub fn debug(&self) -> &SimpleAction {
        &self.debug
    }

    pub fn profile(&self) -> &SimpleAction {
        &self.profile
    }

    pub fn quit(&self) -> &SimpleAction {
        &self.quit
    }

    pub fn update(&self) -> &SimpleAction {
        &self.update
    }
}
