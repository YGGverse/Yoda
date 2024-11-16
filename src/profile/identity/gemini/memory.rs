mod error;
use error::Error;

use std::{cell::RefCell, collections::HashMap};

/// Reduce disk usage by cache index in memory
pub struct Memory {
    index: RefCell<HashMap<i64, String>>,
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

    /// Add new record with `id` as key and `pem` as value
    /// * validate record with same key does not exist yet
    pub fn add(&self, id: i64, pem: String) -> Result<(), Error> {
        match self.index.borrow_mut().insert(id, pem) {
            Some(_) => Err(Error::Overwrite), // @TODO prevent?
            None => Ok(()),
        }
    }

    /// Get `pem` clone by `id` from memory index
    pub fn get(&self, id: i64) -> Result<String, Error> {
        match self.index.borrow().get(&id) {
            Some(pem) => Ok(pem.clone()),
            None => Err(Error::NotFound),
        }
    }

    /// Cleanup index
    pub fn clear(&self) {
        self.index.borrow_mut().clear()
    }
}
