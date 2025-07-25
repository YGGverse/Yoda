mod database;
mod item;
mod memory;

use anyhow::Result;
use database::Database;
use gtk::glib::DateTime;
use item::Item;
use memory::Memory;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use sqlite::Transaction;
use std::cell::RefCell;

pub struct Bookmark {
    database: Database,      // permanent storage
    memory: RefCell<Memory>, // fast search index
}

impl Bookmark {
    // Constructors

    /// Create new `Self`
    pub fn build(database_pool: &Pool<SqliteConnectionManager>, profile_id: i64) -> Result<Self> {
        // Init children components
        let database = Database::new(database_pool, profile_id);
        let memory = RefCell::new(Memory::new());

        // Build initial index
        {
            let mut memory = memory.borrow_mut();
            for item in database.records(None, None)? {
                memory.add(item);
            }
        }

        // Return new `Self`
        Ok(Self { database, memory })
    }

    // Actions

    /// Toggle bookmark in `database` and `memory` index
    /// * return `true` on bookmark create, `false` on delete
    pub fn toggle(&self, request: &str, title: Option<&str>) -> Result<bool> {
        let mut memory = self.memory.borrow_mut();
        Ok(match memory.delete_by_request(request) {
            Some(item) => {
                self.database.delete(item.id)?;
                false
            }
            None => {
                let time = DateTime::now_local()?;
                memory.add(Item {
                    id: self.database.add(time.clone(), request, title)?,
                    request: request.into(),
                    time,
                    title: title.map(|t| t.to_string()),
                });
                true
            }
        })
    }

    // Getters

    /// Check `request` exists in the memory index
    pub fn is_match_request(&self, request: &str) -> bool {
        self.memory.borrow_mut().is_match_request(request)
    }

    /// Find Items match `request`
    pub fn contains_request(&self, request: &str, limit: Option<usize>) -> Vec<Item> {
        self.memory.borrow_mut().contains_request(request, limit)
    }

    /// Get recent Items vector from `memory`, sorted by `ID` DESC
    pub fn recent(&self, limit: Option<usize>) -> Vec<Item> {
        self.memory.borrow().recent(limit)
    }
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
