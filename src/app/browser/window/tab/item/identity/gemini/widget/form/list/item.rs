mod imp;
pub mod value;

use gtk::glib::{self, Object};
use value::Value;

glib::wrapper! {
    pub struct Item(ObjectSubclass<imp::Item>);
}

// C-type property `value` conversion for `Item`
// * values > 0 reserved for `profile_identity_gemini_id`
const G_VALUE_GENERATE_NEW_AUTH: i64 = 0;
const G_VALUE_USE_GUEST_SESSION: i64 = -1;

impl Item {
    // Constructors

    /// Create new `GObject`
    pub fn new(value: Value, title: &str, subtitle: &str) -> Self {
        Object::builder()
            .property(
                "value",
                match value {
                    Value::GENERATE_NEW_AUTH => G_VALUE_GENERATE_NEW_AUTH,
                    Value::USE_GUEST_SESSION => G_VALUE_USE_GUEST_SESSION,
                    Value::PROFILE_IDENTITY_GEMINI_ID(value) => value,
                },
            )
            .property("title", title)
            .property("subtitle", subtitle)
            .build()
    }

    // Getters

    /// Get `value` as enum `Value`
    pub fn value_enum(&self) -> Value {
        match self.value() {
            G_VALUE_GENERATE_NEW_AUTH => Value::GENERATE_NEW_AUTH,
            G_VALUE_USE_GUEST_SESSION => Value::USE_GUEST_SESSION,
            value => Value::PROFILE_IDENTITY_GEMINI_ID(value),
        }
    }
}
