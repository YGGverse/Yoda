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

    /// Get record by `ID`
    pub fn records(&self) -> Vec<Row> {
        self.index.borrow().clone()
    }
}
