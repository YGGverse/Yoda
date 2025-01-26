use gtk::{gio::SimpleAction, glib::uuid_string_random};

pub trait Home {
    fn home() -> Self;
}

impl Home for SimpleAction {
    fn home() -> Self {
        let home = SimpleAction::new(&uuid_string_random(), None);
        home.set_enabled(false);
        home
    }
}
