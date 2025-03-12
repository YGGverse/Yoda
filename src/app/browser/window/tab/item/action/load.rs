use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, GString},
    prelude::{ActionExt, StaticVariantType, ToVariant},
};

/// [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) wrapper for `Load` action of `Item` group
pub struct Load {
    pub simple_action: SimpleAction,
}

impl Default for Load {
    fn default() -> Self {
        Self::new()
    }
}

impl Load {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            simple_action: SimpleAction::new(
                &uuid_string_random(),
                Some(&<(String, bool)>::static_variant_type()),
            ),
        }
    }

    // Actions

    /// Emit [activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    /// with formatted for this action [Variant](https://docs.gtk.org/glib/struct.Variant.html) value
    pub fn activate(&self, request: Option<&str>) {
        self.simple_action
            .activate(Some(&(request.unwrap_or_default()).to_variant()));
    }

    // Events

    /// Define callback function for
    /// [SimpleAction::activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    pub fn connect_activate(&self, callback: impl Fn(Option<GString>) + 'static) {
        self.simple_action.connect_activate(move |_, this| {
            let request = this
                .expect("Expected `request` variant")
                .get::<String>()
                .expect("Parameter type does not match `String`");
            callback(match request.is_empty() {
                true => None,
                false => Some(request.into()),
            })
        });
    }
}
