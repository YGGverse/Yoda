mod database;
mod memory;

use anyhow::Result;
use database::Database;
use memory::Memory;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::{cell::RefCell, collections::HashMap};

pub struct Misc {
    database: Database,
    memory: RefCell<HashMap<String, Memory>>,
}

impl Misc {
    // Constructors

    pub fn init(database_pool: &Pool<SqliteConnectionManager>, profile_id: i64) -> Result<Self> {
        let database = Database::init(database_pool, profile_id);

        let rows = database.rows()?;
        let memory = RefCell::new(HashMap::with_capacity(rows.len()));

        {
            // build in-memory index...
            let mut m = memory.borrow_mut();
            // create initial preset (populate index with the default values)
            let v = Memory::highlight_request_entry(true);
            assert!(m.insert(v.key().to_string(), v).is_none());

            // update values from the DB (if exists)
            for row in rows {
                let v = Memory::from_db_row(&row.key, row.value).unwrap();
                assert!(m.insert(v.key().to_string(), v).is_some());
                // * panics if the DB was malformed or changed unexpectedly
            }
        }

        Ok(Self { database, memory })
    }

    // Setters

    pub fn save(&self) -> Result<()> {
        for (_, m) in self.memory.take() {
            self.database.set(m.into_db_row())?;
        }
        Ok(())
    }

    pub fn set_highlight_request_entry(&self, value: bool) -> Option<Memory> {
        let v = Memory::highlight_request_entry(value);
        self.memory.borrow_mut().insert(v.key().to_string(), v)
    }

    // Getters

    pub fn is_highlight_request_entry(&self) -> bool {
        if let Some(k) = self.memory.borrow().values().next() {
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
