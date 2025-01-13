use gtk::glib::{DateTime, GString, Uri};
use itertools::Itertools;
use std::{cell::RefCell, collections::HashMap};

pub struct Value {
    pub unix_timestamp: i64,
    pub uri: Uri,
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
    pub fn set(&self, uri: Uri) {
        self.index.borrow_mut().insert(
            uri.to_str(),
            Value {
                unix_timestamp: DateTime::now_local().unwrap().to_unix(),
                uri,
            },
        );
    }

    /// Get recent records vector
    /// * sorted by `unix_timestamp` DESC
    pub fn recent(&self) -> Vec<Uri> {
        let mut recent: Vec<Uri> = Vec::new();
        for (_, value) in self
            .index
            .borrow()
            .iter()
            .sorted_by(|a, b| Ord::cmp(&b.1.unix_timestamp, &a.1.unix_timestamp))
        {
            recent.push(value.uri.clone())
        }
        recent
    }

    /// Get records total
    pub fn total(&self) -> usize {
        self.index.borrow().len()
    }
}
