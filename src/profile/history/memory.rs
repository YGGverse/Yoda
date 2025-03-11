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

    /// Get recent Items vector sorted by `closed` DESC
    pub fn recently_closed(&self, limit: Option<usize>) -> Vec<Item> {
        let mut recent: Vec<Item> = Vec::new();
        for (i, item) in self
            .0
            .iter()
            .filter(|x| x.closed.is_some())
            .sorted_by(|a, b| {
                Ord::cmp(
                    &b.closed.as_ref().unwrap().time,
                    &a.closed.as_ref().unwrap().time,
                )
            })
            .enumerate()
        {
            if limit.is_some_and(|l| i > l) {
                break;
            }
            recent.push(item.clone())
        }
        recent
    }

    /// Get recent Items vector sorted by `opened` DESC
    pub fn recently_opened(&self, limit: Option<usize>) -> Vec<Item> {
        let mut recent: Vec<Item> = Vec::new();
        for (i, item) in self
            .0
            .iter()
            .sorted_by(|a, b| Ord::cmp(&b.opened.time, &a.opened.time))
            .enumerate()
        {
            if limit.is_some_and(|l| i > l) {
                break;
            }
            recent.push(item.clone())
        }
        recent
    }

    /// Get unordered Items vector contains `request`
    /// * this function is case insensitive
    pub fn contains_request(&self, request: &str, limit: Option<usize>) -> Vec<Item> {
        let mut items: Vec<Item> = Vec::new();
        for (i, item) in self.0.iter().enumerate() {
            if limit.is_some_and(|l| i > l) {
                break;
            }
            let p = request.to_lowercase();
            if item.request.to_lowercase().contains(&p)
                || item
                    .title
                    .as_ref()
                    .is_some_and(|t| t.to_lowercase().contains(&p))
            {
                items.push(item.clone())
            }
        }
        items
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
