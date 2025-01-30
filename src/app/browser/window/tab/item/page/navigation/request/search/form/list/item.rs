mod imp;
pub mod value;

use gtk::glib::{self, Object, Uri, UriFlags};
pub use value::Value;

glib::wrapper! {
    pub struct Item(ObjectSubclass<imp::Item>);
}

// C-type property `value` conversion for `Item`
// * values > 0 reserved for `profile_search_id`
const G_VALUE_ADD: i64 = 0;

impl Item {
    // Constructors

    pub fn add() -> Self {
        Object::builder()
            .property("value", G_VALUE_ADD)
            .property("title", "Add new..")
            .property("is-default", false)
            .build()
    }

    pub fn profile_search_id(profile_search_id: i64, query: &str, is_default: bool) -> Self {
        Object::builder()
            .property("value", profile_search_id)
            .property(
                "title",
                Uri::parse(query, UriFlags::NONE).unwrap().host().unwrap(),
            ) // @TODO handle
            .property("is-default", is_default)
            .build()
    }

    // Getters

    /// Get `Self` C-value as `Value`
    pub fn value_enum(&self) -> Value {
        match self.value() {
            G_VALUE_ADD => Value::Add,
            value => Value::ProfileSearchId(value),
        }
    }
}
