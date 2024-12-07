use gtk::{
    gio::SimpleAction,
    glib::uuid_string_random,
    prelude::{ActionExt, StaticVariantType, ToVariant},
};

/// [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) wrapper for `Update` action
pub struct Update {
    pub gobject: SimpleAction,
}

impl Update {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            gobject: SimpleAction::new(&uuid_string_random(), Some(&bool::static_variant_type())),
        }
    }

    // Actions

    /// Emit [activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    /// with formatted for this action [Variant](https://docs.gtk.org/glib/struct.Variant.html) value
    pub fn activate(&self, is_reload_request: bool) {
        self.gobject.activate(Some(&is_reload_request.to_variant()));
    }

    // Events

    /// Define callback function for
    /// [SimpleAction::activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    pub fn connect_activate(&self, callback: impl Fn(bool) + 'static) {
        self.gobject.connect_activate(move |_, this| {
            callback(
                this.expect("Expected `is_reload_request` variant")
                    .get::<bool>()
                    .expect("Parameter type does not match `bool` type"),
            )
        });
    }
}
