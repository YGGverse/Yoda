pub mod error;
pub use error::Error;

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
            Some(key) => Err(Error::Overwrite(key)), // @TODO prevent?
            None => Ok(()),
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

    /// Get `profile_identity_gemini_id` vector match given `request`
    /// * [Client certificates specification](https://geminiprotocol.net/docs/protocol-specification.gmi#client-certificates)
    /// * contain unspecified length priority implementation @TODO
    pub fn match_priority(&self, request: &str) -> Option<i64> {
        let mut result = Vec::new();

        // Get all records starts with URL cached, collect length for priority
        for (url, &profile_identity_gemini_id) in self.index.borrow().iter() {
            if request.starts_with(url) {
                result.push((profile_identity_gemini_id, url.len()))
            }
        }

        // Sort by length desc @TODO
        result.sort_by(|a, b| b.1.cmp(&a.1));

        // Get first match ID
        result.first().map(|value| value.0)
    }
}
