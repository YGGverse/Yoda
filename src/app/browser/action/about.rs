use gtk::{gio::SimpleAction, glib::uuid_string_random};

/// [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) wrapper for `About` action of `Browser` group
pub struct About {
    pub simple_action: SimpleAction,
}

impl About {
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
