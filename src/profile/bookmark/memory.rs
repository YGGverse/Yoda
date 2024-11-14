use gtk::glib::DateTime;
use std::{cell::RefCell, collections::HashMap};

/// Reduce disk usage by cache results in memory
pub struct Memory {
    index: RefCell<HashMap<String, DateTime>>,
}

impl Memory {
    // Constructors

    pub fn new() -> Self {
        Self {
            index: RefCell::new(HashMap::new()),
        }
    }

    // Actions

    /// Set new record
    /// * replace existing record with new value
    pub fn set(&self, request: String, time: DateTime) {
        // Borrow index to update
        let mut index = self.index.borrow_mut();

        // Cleanup previous record
        if index.get(&request).is_some() {
            index.remove(&request);
        }

        // Insert new record with actual data
        index.insert(request, time);
    }

    pub fn is_exist(&self, request: &str) -> bool {
        self.index.borrow().get(request).is_some()
    }
}
