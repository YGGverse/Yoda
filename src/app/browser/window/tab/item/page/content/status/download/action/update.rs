// Defaults

use gtk::{
    gio::SimpleAction,
    glib::{SignalHandlerId, uuid_string_random},
    prelude::{ActionExt, StaticVariantType, ToVariant},
};

/// Update [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html)
pub struct Update {
    pub action: SimpleAction,
}

impl Default for Update {
    fn default() -> Self {
        Self::new()
    }
}

impl Update {
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
