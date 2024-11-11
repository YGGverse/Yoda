use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, GString},
    prelude::{ActionExt, StaticVariantType, ToVariant},
};

/// [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) wrapper for `Load` action of `Item` group
pub struct Load {
    gobject: SimpleAction,
}

impl Load {
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
    pub fn activate(&self, request: Option<&str>) {
        self.gobject.activate(Some(
            &match request {
                Some(value) => String::from(value),
                None => String::new(),
            }
            .to_variant(),
        ));
    }

    // Events

    /// Define callback function for
    /// [SimpleAction::activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    pub fn connect_activate(&self, callback: impl Fn(Option<GString>) + 'static) {
        self.gobject.connect_activate(move |_, variant| {
            let request = variant
                .expect("Parameter value required")
                .get::<String>()
                .expect("Parameter type does not match `String`");

            callback(match request.is_empty() {
                true => None,
                false => Some(request.into()),
            })
        });
    }

    // Getters

    /// Get reference to [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) GObject
    pub fn gobject(&self) -> &SimpleAction {
        &self.gobject
    }
}
