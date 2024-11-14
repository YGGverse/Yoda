use gtk::{
    gio::SimpleAction,
    glib::uuid_string_random,
    prelude::{ActionExt, StaticVariantType, ToVariant},
};

/// [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) wrapper for `Load` action of `Item` group
pub struct Auth {
    gobject: SimpleAction,
}

impl Auth {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            gobject: SimpleAction::new(&uuid_string_random(), Some(&String::static_variant_type())),
        }
    }

    // Actions

    /// Emit [activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    /// with formatted for this action [Variant](https://docs.gtk.org/glib/struct.Variant.html) value
    pub fn activate(&self, request: &str) {
        self.gobject.activate(Some(&request.to_variant()));
    }

    // Events

    /// Define callback function for
    /// [SimpleAction::activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    pub fn connect_activate(&self, callback: impl Fn(String) + 'static) {
        self.gobject.connect_activate(move |_, this| {
            callback(
                this.expect("Expected variant value")
                    .get::<String>()
                    .expect("Parameter type does not match `String` type"),
            )
        });
    }
}
