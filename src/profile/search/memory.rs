use super::database::Row;
use std::sync::RwLock;

/// Reduce disk usage by cache Bookmarks index in memory
pub struct Memory {
    index: RwLock<Vec<Row>>,
}

impl Memory {
    // Constructors

    /// Create new `Self`
    pub fn init() -> Self {
        Self {
            index: RwLock::new(Vec::new()),
        }
    }

    // Actions

    /// Add new record
    pub fn push(&self, id: i64, query: String, is_default: bool) {
        self.index.write().unwrap().push(Row {
            id,
            query,
            is_default,
        })
    }

    /// Clear all records
    pub fn clear(&self) {
        self.index.write().unwrap().clear()
    }

    // Getters

    /// Get all records
    pub fn records(&self) -> Vec<Row> {
        self.index.read().unwrap().clone()
    }

    /// Get all records
    pub fn default(&self) -> Option<Row> {
        for record in self.index.read().unwrap().iter() {
            if record.is_default {
                return Some(record.clone());
            }
        }
        None
    }
}
