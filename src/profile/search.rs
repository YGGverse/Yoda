mod database;
mod error;
mod memory;

use database::Database;
use error::Error;
use gtk::glib::Uri;
use memory::Memory;
use sqlite::{Connection, Transaction};
use std::{rc::Rc, sync::RwLock};

pub struct Search {
    database: Database, // permanent storage
    memory: Memory,     // fast search index
}

impl Search {
    // Constructors

    /// Create new `Self`
    pub fn build(connection: &Rc<RwLock<Connection>>, profile_id: &Rc<i64>) -> Result<Self, Error> {
        // Init children components
        let database = Database::init(connection, profile_id);
        let memory = Memory::init();

        // Build initial index
        index(&database, &memory)?;

        // Return new `Self`
        Ok(Self { database, memory })
    }

    // Actions

    /// Add new search provider record
    /// * requires valid [Uri](https://docs.gtk.org/glib/struct.Uri.html)
    pub fn add(&self, query: &Uri, is_default: bool) -> Result<(), Error> {
        match self.database.add(query.to_string(), is_default) {
            Ok(_) => Ok(index(&self.database, &self.memory)?),
            Err(e) => Err(Error::Database(e)),
        }
    }
    /// Add new search provider record
    /// * requires valid [Uri](https://docs.gtk.org/glib/struct.Uri.html)
    pub fn set_default(&self, profile_search_id: i64) -> Result<(), Error> {
        match self.database.set_default(profile_search_id) {
            Ok(_) => Ok(index(&self.database, &self.memory)?),
            Err(e) => Err(Error::Database(e)),
        }
    }

    /// Get records from the memory index
    pub fn records(&self) -> Vec<database::Row> {
        self.memory.records()
    }

    /// Delete record from `database` and `memory` index
    pub fn delete(&self, id: i64) -> Result<(), Error> {
        match self.database.delete(id) {
            Ok(_) => Ok(index(&self.database, &self.memory)?),
            Err(e) => Err(Error::Database(e)),
        }
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

/// Sync memory index with database
fn index(database: &Database, memory: &Memory) -> Result<(), Error> {
    memory.clear();
    match database.records() {
        Ok(records) => {
            for record in records {
                memory.push(record.id, record.query, record.is_default)
            }
        }
        Err(e) => return Err(Error::Database(e)),
    }
    Ok(())
}
