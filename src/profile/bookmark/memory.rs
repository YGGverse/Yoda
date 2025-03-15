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

    /// Add new item
    pub fn add(&mut self, item: Item) {
        self.0.push(item)
    }

    /// Delete record from index by `request`
    pub fn delete_by_request(&mut self, request: &str) -> Option<Item> {
        for (i, item) in self.0.iter().enumerate() {
            if item.request == request {
                return Some(self.0.remove(i));
            }
        }
        None
    }

    /// Check `request` exists in the memory index
    pub fn is_match_request(&self, request: &str) -> bool {
        for item in self.0.iter() {
            if item.request == request {
                return true;
            }
        }
        false
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

    /// Get recent Items vector sorted by `ID` DESC
    pub fn recent(&self, limit: Option<usize>) -> Vec<Item> {
        self.0
            .iter()
            .sorted_by(|a, b| Ord::cmp(&b.id, &a.id))
            .take(limit.unwrap_or(usize::MAX))
            .cloned()
            .collect()
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
