use gtk::{gio::SimpleAction, glib::uuid_string_random};

pub trait Identity {
    fn identity() -> Self;
}

impl Identity for SimpleAction {
    fn identity() -> Self {
        let identity = SimpleAction::new(&uuid_string_random(), None);
        identity.set_enabled(false);
        identity
    }
}
