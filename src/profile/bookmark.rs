mod database;
mod memory;

use anyhow::Result;
use database::Database;
use gtk::glib::DateTime;
use memory::Memory;
use sqlite::{Connection, Transaction};
use std::{rc::Rc, sync::RwLock};

pub struct Bookmark {
    pub database: Rc<Database>, // permanent storage
    pub memory: Rc<Memory>,     // fast search index
}

impl Bookmark {
    // Constructors

    /// Create new `Self`
    pub fn build(connection: &Rc<RwLock<Connection>>, profile_id: &Rc<i64>) -> Result<Self> {
        // Init children components
        let database = Rc::new(Database::new(connection, profile_id));
        let memory = Rc::new(Memory::new());

        // Build initial index
        for record in database.records(None)? {
            memory.add(record.request, record.id)?;
        }

        // Return new `Self`
        Ok(Self { database, memory })
    }

    // Actions

    /// Get record `id` by `request` from memory index
    pub fn get(&self, request: &str) -> Option<i64> {
        self.memory.get(request)
    }

    /// Toggle record in `database` and `memory` index
    /// * return `true` on bookmark created, `false` on deleted
    pub fn toggle(&self, request: &str) -> Result<bool> {
        // Delete record if exists
        if let Some(id) = self.get(request) {
            match self.database.delete(id) {
                Ok(_) => match self.memory.delete(request) {
                    Ok(_) => Ok(false),
                    Err(_) => panic!(), // unexpected
                },
                Err(_) => panic!(), // unexpected
            }
        // Otherwise, create new record
        } else {
            match self
                .database
                .add(DateTime::now_local().unwrap(), request.into())
            {
                Ok(id) => match self.memory.add(request.into(), id) {
                    Ok(_) => Ok(true),
                    Err(_) => panic!(), // unexpected
                },
                Err(_) => panic!(), // unexpected
            }
        } // @TODO return affected rows on success?
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
