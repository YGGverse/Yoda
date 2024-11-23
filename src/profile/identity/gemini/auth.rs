//! Controller for children `database` and `memory` components

mod database;
mod error;
mod memory;

use database::Database;
pub use error::Error;
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

    /// Apply `profile_identity_gemini_id` certificate as the auth for `url`
    /// * deactivate active auth by remove previous records from `Self` database
    /// * reindex `Self` memory index on success
    /// * return last insert `profile_identity_gemini_auth_id` on success
    pub fn apply(&self, profile_identity_gemini_id: i64, url: &str) -> Result<i64, Error> {
        // Cleanup records match `url` (unauthorize)
        self.remove(url)?;

        // Create new record (auth)
        let profile_identity_gemini_auth_id =
            match self.database.add(profile_identity_gemini_id, url) {
                Ok(id) => id,
                Err(reason) => return Err(Error::Database(reason)),
            };

        // Reindex
        self.index()?;

        // Done
        Ok(profile_identity_gemini_auth_id)
    }

    /// Remove all records match request (unauthorize)
    pub fn remove(&self, url: &str) -> Result<(), Error> {
        match self.database.records(Some(url)) {
            Ok(records) => {
                for record in records {
                    if let Err(reason) = self.database.delete(record.id) {
                        return Err(Error::Database(reason));
                    }
                }
            }
            Err(reason) => return Err(Error::Database(reason)),
        }
        self.index()?;
        Ok(())
    }

    /// Create new `Memory` index from `Database` for `Self`
    pub fn index(&self) -> Result<(), Error> {
        // Clear previous records
        if let Err(reason) = self.memory.clear() {
            return Err(Error::Memory(reason));
        }

        // Build new index
        match self.database.records(None) {
            Ok(records) => {
                for record in records {
                    if let Err(reason) = self
                        .memory
                        .add(record.url, record.profile_identity_gemini_id)
                    {
                        return Err(Error::Memory(reason));
                    }
                }
            }
            Err(reason) => return Err(Error::Database(reason)),
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
