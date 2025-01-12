use gtk::{gio::SimpleAction, glib::uuid_string_random, prelude::ActionExt};

/// [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) wrapper for `Ident` action of `Item` group
pub struct Ident {
    pub simple_action: SimpleAction,
}

impl Default for Ident {
    fn default() -> Self {
        Self::new()
    }
}

impl Ident {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            simple_action: SimpleAction::new(&uuid_string_random(), None),
        }
    }

    // Actions

    /// Emit [activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    /// with formatted for this action [Variant](https://docs.gtk.org/glib/struct.Variant.html) value
    pub fn activate(&self) {
        self.simple_action.activate(None);
    }

    // Events

    /// Define callback function for
    /// [SimpleAction::activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    pub fn connect_activate(&self, callback: impl Fn() + 'static) {
        self.simple_action.connect_activate(move |_, _| callback());
    }
}
