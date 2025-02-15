// Defaults

use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, SignalHandlerId},
    prelude::StaticVariantType,
};

/// Load [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html)
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
                Some(&String::static_variant_type()),
            ),
        }
    }

    // Actions

    /// Formatted action connector for external implementation
    pub fn on_activate(
        &self,
        callback: impl Fn(&SimpleAction, String) + 'static,
    ) -> SignalHandlerId {
        self.simple_action.connect_activate(move |this, message| {
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
