mod item;
use item::Item;

use gtk::glib::GString;
use std::cell::{Cell, RefCell};

pub struct Redirect {
    index: RefCell<Vec<Item>>,
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
            index: RefCell::new(Vec::new()),
        }
    }

    // Actions

    pub fn add(&self, request: GString, referrer: Option<GString>, is_foreground: bool) -> &Self {
        self.index.borrow_mut().push(Item {
            request,
            referrer,
            is_foreground,
            is_processed: Cell::new(false),
        });
        self
    }

    pub fn clear(&self) {
        self.index.borrow_mut().clear()
    }

    // Getters

    pub fn count(&self) -> usize {
        self.index.borrow().len() + 1
    }

    pub fn last(&self) -> Option<Item> {
        if let Some(redirect) = self.index.borrow().last() {
            if !redirect.is_processed.replace(true) {
                return Some(redirect.clone());
            }
        }
        None
    }
}
