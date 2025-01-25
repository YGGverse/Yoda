use gtk::{gio::SimpleAction, glib::uuid_string_random};

pub trait Back {
    fn back() -> Self;
}

impl Back for SimpleAction {
    fn back() -> Self {
        let back = SimpleAction::new(&uuid_string_random(), None);
        back.set_enabled(false);
        back
    }
}
