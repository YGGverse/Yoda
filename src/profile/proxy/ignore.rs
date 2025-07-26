mod database;
mod memory;

use anyhow::Result;
use database::Database;
use memory::Memory;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::cell::RefCell;

pub struct Ignore {
    database: Database,
    memory: RefCell<Vec<Memory>>,
}

impl Ignore {
    // Constructors

    pub fn init(database_pool: &Pool<SqliteConnectionManager>, profile_id: i64) -> Result<Self> {
        let database = Database::init(database_pool, profile_id);

        let rows = database.rows()?;
        let memory = RefCell::new(Vec::with_capacity(rows.len()));

        {
            // build in-memory index...
            let mut m = memory.borrow_mut();
            for i in rows {
                m.push(Memory {
                    is_enabled: i.is_enabled,
                    host: i.host,
                });
            }
        }

        Ok(Self { database, memory })
    }

    // Actions

    // Getters

    pub fn all(&self) -> Vec<Memory> {
        self.memory.borrow().iter().cloned().collect()
    }

    pub fn enabled(&self) -> Vec<Memory> {
        self.memory
            .borrow()
            .iter()
            .filter(|r| r.is_enabled)
            .cloned()
            .collect()
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
