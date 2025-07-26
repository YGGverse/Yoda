mod database;
mod memory;

use anyhow::Result;
use database::Database;
use gtk::glib::DateTime;
use memory::Memory;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::cell::RefCell;

pub struct Rule {
    database: Database,
    memory: RefCell<Vec<Memory>>,
}

impl Rule {
    // Constructors

    pub fn init(database_pool: &Pool<SqliteConnectionManager>, profile_id: i64) -> Result<Self> {
        let database = Database::init(database_pool, profile_id);

        let rows = database.rows()?;
        let memory = RefCell::new(Vec::with_capacity(rows.len()));

        {
            // build in-memory index...
            let mut m = memory.borrow_mut();
            for row in rows {
                m.push(Memory {
                    id: Some(row.id),
                    time: row.time,
                    is_enabled: row.is_enabled,
                    priority: row.priority,
                    request: row.request,
                    url: row.url,
                });
            }
        }

        Ok(Self { database, memory })
    }

    // Setters

    pub fn add(
        &self,
        id: Option<i64>,
        is_enabled: bool,
        priority: i32,
        request: String,
        url: String,
        time: DateTime,
    ) {
        let mut rules = self.memory.borrow_mut();
        rules.push(Memory {
            id,
            time,
            is_enabled,
            priority,
            request,
            url,
        }) // @TODO validate?
    }

    pub fn clear(&self) {
        self.memory.borrow_mut().clear();
    }

    pub fn save(&self) -> Result<()> {
        let rules = self.memory.take();
        let mut keep_id = Vec::with_capacity(rules.len());
        for rule in rules {
            keep_id.push(self.database.persist(
                rule.id,
                rule.time,
                rule.is_enabled,
                rule.priority,
                rule.request,
                rule.url,
            )?);
        }
        self.database.clean(keep_id)?;
        Ok(())
    }

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
