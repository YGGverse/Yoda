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
            if memory.add(record.request, record.id).is_err() {
                todo!()
            }
        }

        // Return new `Self`
        Self { database, memory }
    }

    // Actions

    /// Get record `id` by `request` from memory index
    pub fn get(&self, request: &str) -> Result<i64, Error> {
        match self.memory.get(request) {
            Ok(id) => Ok(id),
            Err(_) => Err(Error::MemoryNotFound),
        }
    }

    /// Toggle record in `database` and `memory` index
    pub fn toggle(&self, request: &str) -> Result<(), Error> {
        // Delete record if exists
        if let Ok(id) = self.get(request) {
            match self.database.delete(id) {
                Ok(_) => match self.memory.delete(request) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(Error::MemoryDelete),
                },
                Err(_) => Err(Error::DatabaseDelete),
            }
        // Otherwise, create new record
        } else {
            match self
                .database
                .add(DateTime::now_local().unwrap(), request.into())
            {
                Ok(id) => match self.memory.add(request.into(), id) {
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
