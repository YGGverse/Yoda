pub mod auth;
pub mod error;

pub use auth::Auth;
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
        // Borrow shared index access
        let mut index = self.index.borrow_mut();

        // Prevent existing key overwrite
        if index.contains_key(&url) {
            return Err(Error::Overwrite(url));
        }

        // Slot should be free, let check it twice
        match index.insert(url, profile_identity_gemini_id) {
            Some(_) => Err(Error::Unexpected),
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

    /// Get identity match `request`
    /// * [Client certificates specification](https://geminiprotocol.net/docs/protocol-specification.gmi#client-certificates)
    /// * contain unspecified length priority implementation @TODO
    pub fn match_priority(&self, request: &str) -> Option<Auth> {
        let mut result = Vec::new();

        // Get all records starts with URL cached, collect length for priority
        for (scope, &profile_identity_gemini_id) in self.index.borrow().iter() {
            if request.starts_with(scope) {
                result.push(Auth {
                    profile_identity_gemini_id,
                    scope: scope.clone(),
                })
            }
        }

        // Sort by length desc @TODO
        result.sort_by(|a, b| b.scope.len().cmp(&a.scope.len()));

        // Get first copy
        result.first().cloned()
    }
}
