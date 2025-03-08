use super::Item;
use itertools::Itertools;

/// Reduce disk usage by cache Bookmarks index in memory
pub struct Memory(Vec<Item>);

impl Memory {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self(Vec::new())
    }

    // Actions

    pub fn add(&mut self, item: Item) {
        self.0.push(item)
    }

    /// Update `opened` time for given `request`
    /// * return `false` if the `request` not found in memory index
    pub fn open(&mut self, request: &str) -> bool {
        for record in &mut self.0 {
            if record.request == request {
                record.open();
                return true;
            }
        }
        false
    }

    /// Update `closed` time for given `request`
    pub fn close(&mut self, request: &str) {
        for record in &mut self.0 {
            if record.request == request {
                record.close();
                return;
            }
        }
    }

    // Getters

    /// Get recent Items vector sorted by `closed` ASC
    pub fn recently_closed(&self, limit: Option<usize>) -> Vec<Item> {
        let mut recent: Vec<Item> = Vec::new();
        for (i, item) in self
            .0
            .iter()
            .filter(|x| x.closed.is_some())
            .sorted_by(|a, b| Ord::cmp(&a.closed, &b.closed))
            .enumerate()
        {
            if limit.is_some_and(|l| i > l) {
                break;
            }
            recent.push(item.clone())
        }
        recent
    }

    /// Get recent Items vector sorted by `opened` ASC
    pub fn recently_opened(&self, limit: Option<usize>) -> Vec<Item> {
        let mut recent: Vec<Item> = Vec::new();
        for (i, item) in self
            .0
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.opened, &b.opened))
            .enumerate()
        {
            if limit.is_some_and(|l| i > l) {
                break;
            }
            recent.push(item.clone())
        }
        recent
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
