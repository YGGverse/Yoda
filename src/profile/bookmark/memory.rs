use anyhow::Result;
use itertools::Itertools;
use std::{cell::RefCell, collections::HashMap};

/// Reduce disk usage by cache Bookmarks index in memory
pub struct Memory {
    index: RefCell<HashMap<String, i64>>,
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

    /// Add new record with `request` as key and `id` as value
    /// * validate record with same key does not exist yet
    pub fn add(&self, request: String, id: i64) -> Result<()> {
        // Borrow shared index access
        let mut index = self.index.borrow_mut();

        // Prevent existing key overwrite
        if index.contains_key(&request) {
            panic!() // unexpected
        }

        // Slot should be free, let check it twice
        match index.insert(request, id) {
            Some(_) => panic!(), // unexpected
            None => Ok(()),
        }
    }

    /// Delete record from index by `request`
    /// * validate record key is exist
    pub fn delete(&self, request: &str) -> Result<()> {
        match self.index.borrow_mut().remove(request) {
            Some(_) => Ok(()),
            None => panic!(), // unexpected
        }
    }

    /// Get `id` by `request` from memory index
    pub fn get(&self, request: &str) -> Option<i64> {
        self.index.borrow().get(request).copied()
    }

    /// Get recent requests vector sorted by `ID` DESC
    pub fn recent(&self) -> Vec<String> {
        let mut recent: Vec<String> = Vec::new();
        for (request, _) in self
            .index
            .borrow()
            .iter()
            .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
        {
            recent.push(request.to_string())
        }
        recent
    }
}
