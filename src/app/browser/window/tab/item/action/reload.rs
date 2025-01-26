use gtk::{gio::SimpleAction, glib::uuid_string_random};

pub trait Reload {
    fn reload() -> Self;
}

impl Reload for SimpleAction {
    fn reload() -> Self {
        let reload = SimpleAction::new(&uuid_string_random(), None);
        reload.set_enabled(false);
        reload
    }
}
