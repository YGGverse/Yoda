use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, GString},
    prelude::ActionExt,
};

/// [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) wrapper for `Append` action of `Browser` group
pub struct Append {
    gobject: SimpleAction,
}

impl Append {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            gobject: SimpleAction::new(&uuid_string_random(), None),
        }
    }

    // Actions

    /// Emit [activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    pub fn activate(&self) {
        self.gobject.activate(None);
    }

    // Events

    /// Define callback function for
    /// [SimpleAction::activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    pub fn connect_activate(&self, callback: impl Fn() + 'static) {
        self.gobject.connect_activate(move |_, _| callback());
    }

    // Getters

    /// Get reference to [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) GObject
    pub fn gobject(&self) -> &SimpleAction {
        &self.gobject
    }

    /// Get auto-generated [action name](https://docs.gtk.org/gio/property.SimpleAction.name.html)
    pub fn id(&self) -> GString {
        self.gobject.name()
    }
}
