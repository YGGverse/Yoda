mod request;
mod tab;

use request::Request;
use tab::Tab;

/// Reduce disk usage by cache Bookmarks index in memory
pub struct Memory {
    pub request: Request,
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
        Self {
            request: Request::new(),
            tab: Tab::new(),
        }
    }
}
