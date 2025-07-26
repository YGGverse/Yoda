mod database;
mod memory;

use anyhow::Result;
use database::Database;
use memory::Memory;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::{cell::RefCell, collections::HashSet};

pub struct Misc {
    database: Database,
    memory: RefCell<HashSet<Memory>>,
}

impl Misc {
    // Constructors

    pub fn init(database_pool: &Pool<SqliteConnectionManager>, profile_id: i64) -> Result<Self> {
        let database = Database::init(database_pool, profile_id);

        let rows = database.rows()?;
        let memory = RefCell::new(HashSet::with_capacity(rows.len()));

        {
            // build in-memory index...
            let mut m = memory.borrow_mut();
            // create initial preset (populate index with the default values)
            assert!(m.insert(Memory::highlight_request_entry(true)));

            // update values from the DB (if exists)
            for row in rows {
                assert!(!m.insert(Memory::from_db_row(&row.key, row.value).unwrap()));
                // * panics if the DB was malformed or changed unexpectedly
            }
        }

        Ok(Self { database, memory })
    }

    // Setters

    pub fn save(&self) -> Result<()> {
        for k in self.memory.take() {
            self.database.set(k.into_db_row())?;
        }
        Ok(())
    }

    pub fn set_highlight_request_entry(&self, value: bool) -> bool {
        self.memory
            .borrow_mut()
            .insert(Memory::highlight_request_entry(value))
    }

    // Getters

    pub fn is_highlight_request_entry(&self) -> bool {
        if let Some(k) = self.memory.borrow().iter().next() {
            match k {
                Memory::HighlightRequestEntry(v) => return v.is_true(),
            }
        }
        false
    }
}

// Tools

pub fn migrate(tx: &sqlite::Transaction) -> Result<()> {
    // Migrate self components
    database::init(tx)?;

    // Delegate migration to childs
    // nothing yet...

    // Success
    Ok(())
}
