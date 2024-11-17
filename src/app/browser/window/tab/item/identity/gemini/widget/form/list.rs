use gtk::{gio::ListStore, prelude::ObjectExt, DropDown, Label};

const PROPERTY_KEY_NAME: &str = "key"; // Store item key as GTK property
const PROPERTY_KEY_NONE_VALUE: i64 = -1; // C-type conversion for `None` values

pub struct List {
    gobject: DropDown,
    model: ListStore,
}

impl List {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        let model = ListStore::new::<Label>();
        let gobject = DropDown::builder().model(&model).build();

        Self { model, gobject }
    }

    // Actions

    /// Append new item with `profile_identity_gemini_id` as `key` and name as `value`
    pub fn append(&self, key: Option<i64>, value: &str) {
        // Create new label for item
        let item = Label::new(Some(value));

        // Store key as property
        item.set_property(
            PROPERTY_KEY_NAME,
            match key {
                Some(key) => key,
                None => PROPERTY_KEY_NONE_VALUE,
            },
        );

        // Set value as label
        item.set_label(value);

        // Append formatted record
        self.model.append(&item);
    }

    /* @TODO not in use
    /// Get selected `key` or panic on selection not found
    /// * return `None` if current selection key match `PROPERTY_KEY_NONE_VALUE`
    pub fn selected(&self) -> Option<i64> {
        selected(&self.gobject)
    }*/

    // Events

    /// Run callback function on `connect_selected_notify`
    /// * return formatted key as result
    pub fn on_select(&self, callback: impl Fn(Option<i64>) + 'static) {
        self.gobject
            .connect_selected_notify(move |this| callback(selected(this)));
    }

    // Getters

    pub fn gobject(&self) -> &DropDown {
        &self.gobject
    }
}

// Tools

/// Get selected `key` or panic on selection not found
/// * return `None` if current selection key match `PROPERTY_KEY_NONE_VALUE`
fn selected(list: &DropDown) -> Option<i64> {
    match list.selected_item() {
        Some(this) => {
            // Convert back from C-based GObject type
            let key = this.property::<i64>(PROPERTY_KEY_NAME);

            if key == PROPERTY_KEY_NONE_VALUE {
                None
            } else {
                Some(key)
            }
        }
        None => panic!(),
    }
}
