mod database;
mod item;
mod memory;

use anyhow::Result;
use database::Database;
use gtk::glib::GString;
use item::{Event, Item};
use memory::Memory;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use sqlite::Transaction;
use std::sync::RwLock;

pub struct History {
    database: Database,     // permanent storage
    memory: RwLock<Memory>, // fast search index
}

impl History {
    // Constructors

    /// Create new `Self`
    pub fn build(database_pool: &Pool<SqliteConnectionManager>, profile_id: i64) -> Result<Self> {
        // Init children components
        let database = Database::build(database_pool, profile_id);
        let memory = RwLock::new(Memory::new());

        for item in database.records(None, None)? {
            memory.write().unwrap().add(item)
        }

        // Return new `Self`
        Ok(Self { database, memory })
    }

    // Actions

    pub fn save(&self) -> Result<()> {
        for item in self.memory.read().unwrap().items() {
            if !item.is_saved {
                match item.id {
                    Some(_) => {
                        self.database.update(item)?;
                    }
                    None => {
                        self.database.add(item)?;
                    }
                }
            }
        }
        Ok(())
    }

    // Actions

    /// Create new history record
    pub fn open(&self, request: GString, title: Option<GString>) {
        let mut memory = self.memory.write().unwrap();
        if !memory.open(&request) {
            memory.add(Item {
                id: None,
                request,
                title,
                opened: Event::new(),
                closed: None,
                is_saved: false,
            })
        }
    }

    /// Close existing history record
    pub fn close(&self, request: &str) {
        self.memory.write().unwrap().close(request)
    }

    // Getters

    /// Get recently `opened` Items vector from the memory index, sorted by ASC
    pub fn recently_opened(&self, limit: Option<usize>) -> Vec<Item> {
        self.memory.read().unwrap().recently_opened(limit)
    }

    /// Get recently `closed` Items vector from the memory index, sorted by ASC
    pub fn recently_closed(&self, limit: Option<usize>) -> Vec<Item> {
        self.memory.read().unwrap().recently_closed(limit)
    }

    /// Get unordered Items vector that contains given `request`
    pub fn contains_request(&self, request: &str, limit: Option<usize>) -> Vec<Item> {
        self.memory.read().unwrap().contains_request(request, limit)
    } // @TODO close memory member
}

// Tools

pub fn migrate(tx: &Transaction) -> Result<()> {
    // Migrate self components
    database::init(tx)?;

    // Delegate migration to childs
    // nothing yet..

    // Success
    Ok(())
}
