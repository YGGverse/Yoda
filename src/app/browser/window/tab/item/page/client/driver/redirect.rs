mod item;
use item::Item;

use gtk::glib::Uri;
use std::cell::{Cell, RefCell};

/// Global limit to prevent infinitive redirection issues
/// * defined value is globally applicable to ALL drivers
/// * every driver implement its own value, according to protocol specification
/// * the `Client` will forcefully break redirection loop when iteration reach this value
pub const LIMIT: usize = 10; // @TODO make optional

pub struct Redirect {
    chain: RefCell<Vec<Item>>,
}

impl Default for Redirect {
    fn default() -> Self {
        Self::new()
    }
}

impl Redirect {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            chain: RefCell::new(Vec::new()),
        }
    }

    // Actions

    /// Register new redirect in chain
    pub fn add(&self, request: Uri, referrer: Option<Uri>, is_foreground: bool) -> &Self {
        self.chain.borrow_mut().push(Item {
            request,
            referrer,
            is_foreground,
            is_processed: Cell::new(false),
        });
        self
    }

    /// Clear redirect chain
    pub fn clear(&self) {
        self.chain.borrow_mut().clear()
    }

    // Getters

    /// Get total redirects count in chain
    pub fn count(&self) -> usize {
        self.chain.borrow().len() + 1
    }

    /// Get last redirection `Item` copy
    pub fn last(&self) -> Option<Item> {
        if let Some(redirect) = self.chain.borrow().last() {
            if !redirect.is_processed.replace(true) {
                return Some(redirect.clone());
            }
        }
        None
    }
}
