use gtk::{gio::SimpleAction, glib::uuid_string_random};

/// [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) wrapper for `Profile` action of `Browser` group
pub struct Profile {
    pub simple_action: SimpleAction,
}

impl Default for Profile {
    fn default() -> Self {
        Self::new()
    }
}

impl Profile {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            simple_action: SimpleAction::new(&uuid_string_random(), None),
        }
    }

    // Events

    /// Define callback function for
    /// [SimpleAction::activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    pub fn connect_activate(&self, callback: impl Fn() + 'static) {
        self.simple_action.connect_activate(move |_, _| callback());
    }
}
