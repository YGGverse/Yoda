pub mod error;
pub use error::Error;

use std::{cell::RefCell, collections::HashMap};

/// Reduce disk usage by cache index in memory
pub struct Memory {
    index: RefCell<HashMap<i64, String>>,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
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
    pub fn add(&self, profile_identity_gemini_id: i64, pem: String) -> Result<(), Error> {
        // Borrow shared index access
        let mut index = self.index.borrow_mut();

        // Prevent existing key overwrite
        if index.contains_key(&profile_identity_gemini_id) {
            return Err(Error::Overwrite(profile_identity_gemini_id));
        }

        // Slot should be free, let check it twice
        match index.insert(profile_identity_gemini_id, pem) {
            Some(_) => Err(Error::Unexpected),
            None => Ok(()),
        }
    }

    /// Get `pem` clone by `id` from memory index
    pub fn get(&self, id: i64) -> Result<String, Error> {
        match self.index.borrow().get(&id) {
            Some(pem) => Ok(pem.clone()),
            None => Err(Error::NotFound(id)),
        }
    }

    /// Cleanup index
    pub fn clear(&self) -> Result<(), Error> {
        let mut index = self.index.borrow_mut();
        index.clear();
        if index.is_empty() {
            Ok(())
        } else {
            Err(Error::Clear)
        }
    }
}
