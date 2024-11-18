use gtk::{gio::ListStore, DropDown, Label};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct List {
    pub gobject: DropDown,
    model: ListStore,
    index: Rc<RefCell<HashMap<Label, Option<i64>>>>,
}

impl List {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        let index = Rc::new(RefCell::new(HashMap::new()));
        let model = ListStore::new::<Label>();
        let gobject = DropDown::builder().model(&model).build();

        Self {
            model,
            index,
            gobject,
        }
    }

    // Actions

    /// Append new item with `profile_identity_gemini_id` as `key` and label as `value`
    pub fn append(&self, profile_identity_gemini_id: Option<i64>, label: &str) {
        // Create new label for item
        let item = Label::new(Some(label));

        // Register ID in hash map index
        self.index
            .borrow_mut()
            .insert(item.clone(), profile_identity_gemini_id);

        // Append formatted record
        self.model.append(&item);
    }

    // Events

    /// Run callback function on `connect_selected_notify` event
    /// * return formatted `profile_identity_gemini_id` match selected
    pub fn connect_selected_notify(&self, callback: impl Fn(Option<i64>) + 'static) {
        self.gobject.connect_selected_notify({
            let index = self.index.clone();
            move |list| callback(*index.borrow().get(&list.selected_item().unwrap()).unwrap())
        });
    }
}
