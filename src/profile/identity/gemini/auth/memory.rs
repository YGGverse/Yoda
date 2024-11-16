mod error;
use error::Error;

use std::{cell::RefCell, collections::HashMap};

/// Reduce disk usage by cache Auth index in memory
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

    /// Add new record with `url` as key and `profile_identity_gemini_id` as value
    /// * validate record with same key does not exist yet
    pub fn add(&self, url: String, profile_identity_gemini_id: i64) -> Result<(), Error> {
        match self
            .index
            .borrow_mut()
            .insert(url, profile_identity_gemini_id)
        {
            Some(_) => Err(Error::Overwrite), // @TODO prevent?
            None => Ok(()),
        }
    }

    /* @TODO update feature
    /// Cleanup index
    pub fn clear(&self, url: &str) {
        self.index.borrow_mut().clear()
    } */

    /// Search for `profile_identity_gemini_id` by `url` starts with given substring
    pub fn starts_with(&self, prefix: &str) -> Vec<i64> {
        let mut result = Vec::new();
        for (url, &profile_identity_gemini_id) in self.index.borrow().iter() {
            if url.starts_with(prefix) {
                result.push(profile_identity_gemini_id)
            }
        }
        result
    }
}
