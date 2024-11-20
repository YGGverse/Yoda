mod imp;
use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct Item(ObjectSubclass<imp::Item>);
}

// C-type conversion
const OPTION_NEW: i64 = -1;

impl Item {
    // Constructors

    /// Create new `GObject` with formatted properties
    pub fn new(profile_identity_gemini_id: Option<i64>, title: &str, subtitle: &str) -> Self {
        Object::builder()
            .property(
                "profile_identity_gemini_id",
                match profile_identity_gemini_id {
                    Some(value) => value,
                    None => OPTION_NEW,
                },
            )
            .property("title", title)
            .property("subtitle", subtitle)
            .build()
    }

    // Getters

    /// Additional `profile_identity_gemini_id` wrapper with `Option` value support
    pub fn profile_identity_gemini_id_option(&self) -> Option<i64> {
        match self.profile_identity_gemini_id() {
            OPTION_NEW => None,
            value => Some(value),
        }
    }
}
