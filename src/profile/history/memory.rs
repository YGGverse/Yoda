mod tab;
use tab::Tab;

/// Reduce disk usage by cache Bookmarks index in memory
pub struct Memory {
    pub tab: Tab,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self { tab: Tab::new() }
    }
}
