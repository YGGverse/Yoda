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
const G_VALUE_IMPORT_PEM: i64 = -1;
const G_VALUE_USE_GUEST_SESSION: i64 = -2;

impl Item {
    // Constructors

    /// Create new `GObject`
    pub fn new(value: Value, title: &str, subtitle: &str, is_active: bool) -> Self {
        Object::builder()
            .property(
                "value",
                match value {
                    Value::GenerateNewAuth => G_VALUE_GENERATE_NEW_AUTH,
                    Value::ImportPem => G_VALUE_IMPORT_PEM,
                    Value::UseGuestSession => G_VALUE_USE_GUEST_SESSION,
                    Value::ProfileIdentityGeminiId(value) => value,
                },
            )
            .property("title", title)
            .property("subtitle", subtitle)
            .property("is_active", is_active)
            .build()
    }

    // Getters

    /// Get `value` as enum `Value`
    pub fn value_enum(&self) -> Value {
        match self.value() {
            G_VALUE_GENERATE_NEW_AUTH => Value::GenerateNewAuth,
            G_VALUE_IMPORT_PEM => Value::ImportPem,
            G_VALUE_USE_GUEST_SESSION => Value::UseGuestSession,
            value => Value::ProfileIdentityGeminiId(value),
        }
    }
}
