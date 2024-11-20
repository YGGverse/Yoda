mod imp;
pub mod value;

use gtk::glib::{self, Object};
use value::Value;

glib::wrapper! {
    pub struct Item(ObjectSubclass<imp::Item>);
}

// C-type property `value` conversion for `Item`
const CREATE_NEW_AUTH: i64 = 0;
const REMOVE_CURRENT_AUTH: i64 = -1;

impl Item {
    // Constructors

    /// Create new `GObject` with formatted properties
    pub fn new(value: Value, title: &str, subtitle: &str) -> Self {
        Object::builder()
            .property(
                "value",
                match value {
                    Value::CREATE_NEW_AUTH => CREATE_NEW_AUTH,
                    Value::REMOVE_CURRENT_AUTH => REMOVE_CURRENT_AUTH,
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
            CREATE_NEW_AUTH => Value::CREATE_NEW_AUTH,
            REMOVE_CURRENT_AUTH => Value::REMOVE_CURRENT_AUTH,
            value => Value::PROFILE_IDENTITY_GEMINI_ID(value),
        }
    }
}
