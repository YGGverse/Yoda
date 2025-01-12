// Defaults

use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, SignalHandlerId},
    prelude::{ActionExt, StaticVariantType, ToVariant},
};

/// Complete [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html)
pub struct Complete {
    pub action: SimpleAction,
}

impl Default for Complete {
    fn default() -> Self {
        Self::new()
    }
}

impl Complete {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            action: SimpleAction::new(&uuid_string_random(), Some(&String::static_variant_type())),
        }
    }

    // Actions

    pub fn activate(&self, message: &str) {
        self.action.activate(Some(&message.to_variant()));
    }

    /// Formatted action connector for external implementation
    pub fn on_activate(
        &self,
        callback: impl Fn(&SimpleAction, String) + 'static,
    ) -> SignalHandlerId {
        self.action.connect_activate(move |this, message| {
            callback(
                this,
                message
                    .expect("Variant required to call this action")
                    .get::<String>()
                    .expect("Parameter does not match `String` type"),
            )
        })
    }
}
