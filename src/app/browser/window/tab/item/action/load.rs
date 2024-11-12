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
            gobject: SimpleAction::new(
                &uuid_string_random(),
                Some(&<(String, bool)>::static_variant_type()),
            ),
        }
    }

    // Actions

    /// Emit [activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    /// with formatted for this action [Variant](https://docs.gtk.org/glib/struct.Variant.html) value
    pub fn activate(&self, request: Option<&str>, is_history: bool) {
        self.gobject.activate(Some(
            &(
                match request {
                    Some(value) => String::from(value),
                    None => String::new(),
                },
                is_history,
            )
                .to_variant(),
        ));
    }

    // Events

    /// Define callback function for
    /// [SimpleAction::activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    pub fn connect_activate(&self, callback: impl Fn(Option<GString>, bool) + 'static) {
        self.gobject.connect_activate(move |_, this| {
            let (request, is_history) = this
                .expect("Expected (`request`,`is_history`) variant")
                .get::<(String, bool)>()
                .expect("Parameter type does not match (`String`,`bool`) tuple");

            callback(
                match request.is_empty() {
                    true => None,
                    false => Some(request.into()),
                },
                is_history,
            )
        });
    }
}
