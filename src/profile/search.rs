mod database;
mod memory;

use anyhow::Result;
use database::Database;
use gtk::glib::Uri;
use memory::Memory;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use sqlite::Transaction;

pub struct Search {
    database: Database, // permanent storage
    memory: Memory,     // fast search index
}

impl Search {
    // Constructors

    /// Create new `Self`
    pub fn build(database_pool: &Pool<SqliteConnectionManager>, profile_id: i64) -> Result<Self> {
        let database = Database::init(database_pool, profile_id)?;
        // Init fast search index
        let memory = Memory::init();
        // Build initial index
        index(&database, &memory)?;
        // Return new `Self`
        Ok(Self { database, memory })
    }

    // Actions

    /// Add new search provider record
    /// * requires valid [Uri](https://docs.gtk.org/glib/struct.Uri.html)
    pub fn add(&self, query: &Uri, is_default: bool) -> Result<()> {
        self.database.add(query.to_string(), is_default)?;
        Ok(())
    }

    /// Add new search provider record
    /// * requires valid [Uri](https://docs.gtk.org/glib/struct.Uri.html)
    pub fn set_default(&self, profile_search_id: i64) -> Result<()> {
        self.database.set_default(profile_search_id)?;
        index(&self.database, &self.memory)
    }

    /// Get records from the memory index
    pub fn records(&self) -> Vec<database::Row> {
        self.memory.records()
    }

    /// Delete record from `database` and `memory` index
    pub fn delete(&self, id: i64) -> Result<()> {
        self.database.delete(id)?;
        index(&self.database, &self.memory)
    }

    // Getters

    /// Get default search provider from memory
    pub fn default(&self) -> Option<database::Row> {
        self.memory.default()
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

/// Sync memory index with database
fn index(database: &Database, memory: &Memory) -> Result<()> {
    memory.clear();
    for record in database.records()? {
        memory.push(record.id, record.query, record.is_default)
    }
    Ok(())
}
