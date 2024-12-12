use gtk::{gio::SimpleAction, glib::uuid_string_random, prelude::ActionExt};

/// [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) wrapper for `Focus` action of `Browser` group
pub struct Focus {
    pub gobject: SimpleAction,
}

impl Focus {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            gobject: SimpleAction::new(&uuid_string_random(), None),
        }
    }

    // Actions

    /// Emit [activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    /// with formatted for this action [Variant](https://docs.gtk.org/glib/struct.Variant.html) value
    pub fn activate(&self) {
        self.gobject.activate(None); // @TODO custom value
    }

    // Events

    /// Define callback function for
    /// [SimpleAction::activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    pub fn connect_activate(&self, callback: impl Fn() + 'static) {
        self.gobject.connect_activate(move |_, _| callback());
    }
}
