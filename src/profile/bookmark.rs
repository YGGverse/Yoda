mod database;
mod memory;

use anyhow::Result;
use database::Database;
use gtk::glib::DateTime;
use memory::Memory;
use sqlite::{Connection, Transaction};
use std::{rc::Rc, sync::RwLock};

pub struct Bookmark {
    database: Database, // permanent storage
    memory: Memory,     // fast search index
}

impl Bookmark {
    // Constructors

    /// Create new `Self`
    pub fn build(connection: &Rc<RwLock<Connection>>, profile_id: &Rc<i64>) -> Result<Self> {
        // Init children components
        let database = Database::new(connection, profile_id);
        let memory = Memory::new();

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

    /// Toggle bookmark in `database` and `memory` index
    /// * return `true` on bookmark create, `false` on delete
    pub fn toggle(&self, request: &str) -> Result<bool> {
        Ok(match self.get(request) {
            Some(id) => {
                self.database.delete(id)?;
                self.memory.delete(request)?;
                false
            }
            None => {
                self.memory.add(
                    request.into(),
                    self.database.add(DateTime::now_local()?, request.into())?,
                )?;
                true
            }
        })
    }

    // Getters

    /// Get recent requests vector from `memory`, sorted by `ID` DESC
    pub fn recent(&self) -> Vec<String> {
        self.memory.recent()
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
