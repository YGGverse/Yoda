use super::database::Row;
use std::cell::RefCell;

/// Reduce disk usage by cache Bookmarks index in memory
pub struct Memory {
    index: RefCell<Vec<Row>>,
}

impl Memory {
    // Constructors

    /// Create new `Self`
    pub fn init() -> Self {
        Self {
            index: RefCell::new(Vec::new()),
        }
    }

    // Actions

    /// Add new record
    pub fn push(&self, id: i64, query: String, is_default: bool) {
        self.index.borrow_mut().push(Row {
            id,
            query,
            is_default,
        })
    }

    /// Clear all records
    pub fn clear(&self) {
        self.index.borrow_mut().clear()
    }

    // Getters

    /// Get all records
    pub fn records(&self) -> Vec<Row> {
        self.index.borrow().clone()
    }

    /// Get all records
    pub fn default(&self) -> Option<Row> {
        for record in self.index.borrow().iter() {
            if record.is_default {
                return Some(record.clone());
            }
        }
        None
    }
}
