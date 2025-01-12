mod closed;
use closed::Closed;

/// Reduce disk usage by cache Bookmarks index in memory
pub struct Memory {
    pub closed: Closed,
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
        Self {
            closed: Closed::new(),
        }
    }
}
