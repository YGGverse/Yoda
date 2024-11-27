use gtk::{gio::SimpleAction, glib::uuid_string_random};

/// [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) wrapper for `Debug` action of `Browser` group
pub struct Debug {
    pub gobject: SimpleAction,
}

impl Debug {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            gobject: SimpleAction::new(&uuid_string_random(), None),
        }
    }

    // Events

    /// Define callback function for
    /// [SimpleAction::activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    pub fn connect_activate(&self, callback: impl Fn() + 'static) {
        self.gobject.connect_activate(move |_, _| callback());
    }
}
