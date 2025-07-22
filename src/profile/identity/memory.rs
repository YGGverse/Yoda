use anyhow::{Result, bail};
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
    pub fn add(&self, profile_identity_id: i64, pem: String) -> Result<()> {
        // Borrow shared index access
        let mut index = self.index.borrow_mut();

        // Prevent existing key overwrite
        if index.contains_key(&profile_identity_id) {
            bail!("Overwrite attempt for existing record `{profile_identity_id}`")
        }

        // Slot should be free, let check it twice
        match index.insert(profile_identity_id, pem) {
            Some(_) => bail!("Unexpected error"),
            None => Ok(()),
        }
    }

    /// Get `pem` clone by `id` from memory index
    pub fn get(&self, id: i64) -> Result<String> {
        match self.index.borrow().get(&id) {
            Some(pem) => Ok(pem.clone()),
            None => bail!("Record `{id}` not found in memory index"),
        }
    }

    /// Cleanup index
    pub fn clear(&self) -> Result<()> {
        let mut index = self.index.borrow_mut();
        index.clear();
        if index.is_empty() {
            Ok(())
        } else {
            bail!("Could not cleanup memory index")
        }
    }
}
