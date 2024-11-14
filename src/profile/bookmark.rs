mod database;
mod error;
mod memory;

use database::Database;
use error::Error;
use memory::Memory;

use gtk::glib::DateTime;
use sqlite::{Connection, Transaction};
use std::{rc::Rc, sync::RwLock};

pub struct Bookmark {
    database: Rc<Database>, // permanent storage
    memory: Rc<Memory>,     // fast search index
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
            if memory.add(record.request, record.time).is_err() {
                todo!()
            }
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

    /// Toggle record in `database` and `memory` index
    pub fn toggle(&self, request: &str) -> Result<(), Error> {
        // Get current timestamp for new record
        let time = DateTime::now_local().unwrap();

        // Delete record if exists
        if self.has_request(request, false) {
            match self.database.delete(request) {
                Ok(_) => match self.memory.delete(request) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(Error::MemoryDelete),
                },
                Err(_) => Err(Error::DatabaseDelete),
            }
        // Otherwise, create new record
        } else {
            match self.database.add(time.clone(), request.into()) {
                Ok(_) => match self.memory.add(request.into(), time) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(Error::MemoryAdd),
                },
                Err(_) => Err(Error::DatabaseAdd),
            }
        } // @TODO return affected rows on success?
    }
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
