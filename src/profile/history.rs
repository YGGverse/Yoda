// mod database;
mod item;
mod memory;

use gtk::glib::GString;
use item::Item;
use memory::Memory;
use sqlite::Connection;
use std::{cell::RefCell, rc::Rc, sync::RwLock};

pub struct History {
    memory: RefCell<Memory>, // fast search index
}

impl History {
    // Constructors

    /// Create new `Self`
    pub fn build(_connection: &Rc<RwLock<Connection>>, _profile_id: &Rc<i64>) -> Self {
        // Init children components
        let memory = RefCell::new(Memory::new());

        // Return new `Self`
        Self { memory }
    }

    // Actions

    /// Create new history record
    pub fn open(&self, request: GString, title: Option<GString>) {
        let mut memory = self.memory.borrow_mut();
        if !memory.open(&request) {
            memory.add(Item::init(request, title))
        }
    }

    /// Close existing history record
    pub fn close(&self, request: &str) {
        self.memory.borrow_mut().close(request)
    }

    // Getters

    /// Get recently `opened` Items vector from the memory index, sorted by ASC
    pub fn recently_opened(&self, limit: Option<usize>) -> Vec<Item> {
        self.memory.borrow().recently_opened(limit)
    }

    /// Get recently `closed` Items vector from the memory index, sorted by ASC
    pub fn recently_closed(&self, limit: Option<usize>) -> Vec<Item> {
        self.memory.borrow().recently_closed(limit)
    }

    /// Get unordered Items vector contains `request`
    pub fn contains_request(&self, request: &str, limit: Option<usize>) -> Vec<Item> {
        self.memory.borrow().contains_request(request, limit)
    }
}
