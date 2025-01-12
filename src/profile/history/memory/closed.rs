use gtk::glib::GString;
use itertools::Itertools;
use std::{cell::RefCell, collections::HashMap};

/// Reduce disk usage by cache Bookmarks index in memory
pub struct Closed {
    index: RefCell<HashMap<GString, i64>>,
}

impl Default for Closed {
    fn default() -> Self {
        Self::new()
    }
}

impl Closed {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            index: RefCell::new(HashMap::new()),
        }
    }

    // Actions

    /// Add new record
    /// * replace with new one if the record already exist
    pub fn add(&self, request: GString, unix_timestamp: i64) {
        self.index.borrow_mut().insert(request, unix_timestamp);
    }

    /// Get recent requests vector sorted by `ID` DESC
    pub fn recent(&self, limit: usize) -> Vec<GString> {
        let mut recent: Vec<GString> = Vec::new();
        for (request, _) in self
            .index
            .borrow()
            .iter()
            .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
            .take(limit)
        {
            recent.push(request.clone())
        }
        recent
    }

    /// Get records total
    pub fn total(&self) -> usize {
        self.index.borrow().len()
    }
}
