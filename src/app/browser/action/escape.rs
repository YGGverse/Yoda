use gtk::{gio::SimpleAction, glib::uuid_string_random};

/// [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) wrapper for `Escape` action of `Browser` group
pub trait Escape {
    fn escape() -> Self;
}

impl Escape for SimpleAction {
    fn escape() -> Self {
        SimpleAction::new(&uuid_string_random(), None)
    }
}
