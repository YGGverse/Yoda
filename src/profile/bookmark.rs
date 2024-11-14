mod database;
mod memory;

use database::Database;
use memory::Memory;

use sqlite::{Connection, Transaction};
use std::{rc::Rc, sync::RwLock};

pub struct Bookmark {
    database: Rc<Database>,
    memory: Rc<Memory>,
}

impl Bookmark {
    // Constructors

    /// Create new `Self`
    pub fn new(connection: Rc<RwLock<Connection>>, profile_id: i64) -> Self {
        // Init children components
        let database = Rc::new(Database::new(connection, profile_id));
        let memory = Rc::new(Memory::new());

        // Build initial index
        for record in database.records(None) {
            memory.set(record.request, record.time)
        }

        // Return new `Self`
        Self { database, memory }
    }

    // Actions

    /// Check request exist in:
    /// * memory if `is_memory` is `true` (fast)
    /// * database if `is_memory` is `false` (slow)
    pub fn has_request(&self, request: &str, is_memory: bool) -> bool {
        if is_memory {
            self.memory.is_exist(request)
        } else {
            !self.database.records(Some(request)).is_empty()
        }
    }

    // @TODO add new record with index update
}

// Tools

pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    // nothing yet..

    // Success
    Ok(())
}
