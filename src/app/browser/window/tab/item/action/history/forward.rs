use gtk::{gio::SimpleAction, glib::uuid_string_random};

pub trait Forward {
    fn forward() -> Self;
}

impl Forward for SimpleAction {
    fn forward() -> Self {
        let forward = SimpleAction::new(&uuid_string_random(), None);
        forward.set_enabled(false);
        forward
    }
}
