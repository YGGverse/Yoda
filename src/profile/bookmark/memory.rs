mod error;
use error::Error;

use gtk::glib::DateTime;
use std::{cell::RefCell, collections::HashMap};

/// Reduce disk usage by cache Bookmarks index in memory
pub struct Memory {
    index: RefCell<HashMap<String, DateTime>>,
}

impl Memory {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            index: RefCell::new(HashMap::new()),
        }
    }

    // Actions

    /// Add new record for given `request`
    /// * validates record with same key does not exist yet
    pub fn add(&self, request: String, time: DateTime) -> Result<(), Error> {
        match self.index.borrow_mut().insert(request, time) {
            Some(_) => Err(Error::Overwrite),
            None => Ok(()),
        }
    }

    /// Delete record from index by `request`
    pub fn delete(&self, request: &str) -> Result<(), Error> {
        match self.index.borrow_mut().remove(request) {
            Some(_) => Ok(()),
            None => Err(Error::NotFound),
        }
    }

    /// Check `request` exist in memory index
    pub fn is_exist(&self, request: &str) -> bool {
        self.index.borrow().get(request).is_some()
    }
}
