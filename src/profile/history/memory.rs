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
        for item in &mut self.0 {
            if item.request == request {
                item.open();
                return true;
            }
        }
        false
    }

    /// Update `closed` time for given `request`
    pub fn close(&mut self, request: &str) {
        for item in &mut self.0 {
            if item.request == request {
                item.close();
                return;
            }
        }
    }

    // Getters

    /// Get recent Items vector sorted by `closed` DESC
    pub fn recently_closed(&self, limit: Option<usize>) -> Vec<Item> {
        self.0
            .iter()
            .filter(|x| x.closed.is_some())
            .sorted_by(|a, b| {
                Ord::cmp(
                    &b.closed.as_ref().unwrap().time,
                    &a.closed.as_ref().unwrap().time,
                )
            })
            .take(limit.unwrap_or(usize::MAX))
            .cloned()
            .collect()
    }

    /// Get recent Items vector sorted by `opened` DESC
    pub fn recently_opened(&self, limit: Option<usize>) -> Vec<Item> {
        self.0
            .iter()
            .sorted_by(|a, b| Ord::cmp(&b.opened.time, &a.opened.time))
            .take(limit.unwrap_or(usize::MAX))
            .cloned()
            .collect()
    }

    /// Get unordered Items vector where title or request match `request`
    /// * this function is case insensitive
    pub fn contains_request(&self, request: &str, limit: Option<usize>) -> Vec<Item> {
        self.0
            .iter()
            .filter(|item| {
                let p = request.to_lowercase();
                item.request.to_lowercase().contains(&p)
                    || item
                        .title
                        .as_ref()
                        .is_some_and(|t| t.to_lowercase().contains(&p))
            })
            .take(limit.unwrap_or(usize::MAX))
            .cloned()
            .collect()
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.0
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
