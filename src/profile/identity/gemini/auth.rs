mod database;
mod error;
mod memory;

use database::Database;
use error::Error;
use memory::Memory;

use sqlite::{Connection, Transaction};
use std::{rc::Rc, sync::RwLock};

/// API for `profile_identity_gemini_id` + `url` auth pairs operations
pub struct Auth {
    pub database: Rc<Database>,
    pub memory: Rc<Memory>,
}

impl Auth {
    // Constructors

    /// Create new `Self`
    pub fn new(connection: Rc<RwLock<Connection>>) -> Result<Self, Error> {
        // Init `Self`
        let this = Self {
            database: Rc::new(Database::new(connection)),
            memory: Rc::new(Memory::new()),
        };

        // Build initial index
        Self::index(&this)?;

        // Done
        Ok(this)
    }

    // Actions

    /// Create new `Memory` index from `Database` for `Self`
    pub fn index(&self) -> Result<(), Error> {
        // Clear previous records
        self.memory.clear();

        // Build new index
        match self.database.records(None) {
            Ok(records) => {
                for record in records {
                    if record.is_active {
                        if self
                            .memory
                            .add(record.url, record.profile_identity_gemini_id)
                            .is_err()
                        {
                            return Err(Error::MemoryIndex);
                        }
                    }
                }
            }
            Err(_) => return Err(Error::DatabaseIndex),
        }
        Ok(())
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

    Ok(())
}
