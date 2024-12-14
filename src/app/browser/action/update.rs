use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, GString},
    prelude::{ActionExt, StaticVariantType, ToVariant},
};

/// [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) wrapper for `Update` action of `Browser` group
pub struct Update {
    pub simple_action: SimpleAction,
}

impl Update {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            simple_action: SimpleAction::new(
                &uuid_string_random(),
                Some(&String::static_variant_type()),
            ),
        }
    }

    // Actions

    /// Emit [activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    /// with formatted for this action [Variant](https://docs.gtk.org/glib/struct.Variant.html) value
    pub fn activate(&self, tab_item_id: Option<&str>) {
        self.simple_action.activate(Some(
            &match tab_item_id {
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
        self.simple_action.connect_activate(move |_, variant| {
            let tab_item_id = variant
                .expect("Variant required to call this action")
                .get::<String>()
                .expect("Parameter type does not match `String`");

            callback(match tab_item_id.is_empty() {
                true => None,
                false => Some(tab_item_id.into()),
            })
        });
    }
}
