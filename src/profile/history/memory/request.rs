use gtk::glib::GString;
use itertools::Itertools;
use std::{cell::RefCell, collections::HashMap};

pub struct Value {
    pub unix_timestamp: i64,
}

/// Recent request history
pub struct Request {
    index: RefCell<HashMap<GString, Value>>,
}

impl Default for Request {
    fn default() -> Self {
        Self::new()
    }
}

impl Request {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            index: RefCell::new(HashMap::new()),
        }
    }

    // Actions

    /// Add new record with `request` as key and `unix_timestamp` as value
    /// * replace with new value if `request` already exists
    pub fn set(&self, request: GString, unix_timestamp: i64) {
        self.index
            .borrow_mut()
            .insert(request, Value { unix_timestamp });
    }

    /// Get recent requests vector
    /// * sorted by `unix_timestamp` DESC
    pub fn recent(&self, limit: usize) -> Vec<GString> {
        let mut recent: Vec<GString> = Vec::new();
        for (request, _) in self
            .index
            .borrow()
            .iter()
            .sorted_by(|a, b| Ord::cmp(&b.1.unix_timestamp, &a.1.unix_timestamp))
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
