mod error;
use error::Error;

use std::{cell::RefCell, collections::HashMap};

/// Reduce disk usage by cache Bookmarks index in memory
pub struct Memory {
    index: RefCell<HashMap<String, i64>>,
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

    /// Add new record with `request` as key and `id` as value
    /// * validate record with same key does not exist yet
    pub fn add(&self, request: String, id: i64) -> Result<(), Error> {
        // Borrow shared index access
        let mut index = self.index.borrow_mut();

        // Prevent existing key overwrite
        if index.contains_key(&request) {
            return Err(Error::Overwrite(request));
        }

        // Slot should be free, let check it twice
        match index.insert(request, id) {
            Some(_) => Err(Error::Unexpected),
            None => Ok(()),
        }
    }

    /// Delete record from index by `request`
    /// * validate record key is exist
    pub fn delete(&self, request: &str) -> Result<(), Error> {
        match self.index.borrow_mut().remove(request) {
            Some(_) => Ok(()),
            None => Err(Error::Unexpected), // @TODO
        }
    }

    /// Get `id` by `request` from memory index
    pub fn get(&self, request: &str) -> Result<i64, Error> {
        match self.index.borrow().get(request) {
            Some(&value) => Ok(value),
            None => Err(Error::Unexpected), // @TODO
        }
    }
}
