use crate::app::browser::window::tab::Item;
use itertools::Itertools;
use std::{cell::RefCell, rc::Rc};

pub struct Record {
    pub item: Rc<Item>,
    pub unix_timestamp: i64,
}

/// Recently closed tabs index
pub struct Closed {
    index: RefCell<Vec<Record>>,
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
            index: RefCell::new(Vec::new()),
        }
    }

    // Actions

    /// Add new record
    /// * replace with new one if the record already exist
    pub fn add(&self, item: Rc<Item>, unix_timestamp: i64) {
        self.index.borrow_mut().push(Record {
            item,
            unix_timestamp,
        });
    }

    /// Get recent `Item` vector sorted by time DESC
    pub fn recent(&self, limit: usize) -> Vec<Rc<Item>> {
        let mut recent: Vec<Rc<Item>> = Vec::new();
        for record in self
            .index
            .borrow()
            .iter()
            .sorted_by(|a, b| Ord::cmp(&b.unix_timestamp, &a.unix_timestamp))
            .take(limit)
        {
            recent.push(record.item.clone())
        }
        recent
    }

    /// Get records total
    pub fn total(&self) -> usize {
        self.index.borrow().len()
    }
}
