use gtk::{gio::SimpleAction, glib::uuid_string_random};

pub trait Identity {
    fn identity() -> Self;
}

impl Identity for SimpleAction {
    fn identity() -> Self {
        SimpleAction::new(&uuid_string_random(), None)
    }
}
