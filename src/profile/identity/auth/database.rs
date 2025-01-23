use sqlite::{Connection, Error, Transaction};
use std::{rc::Rc, sync::RwLock};

pub struct Table {
    pub id: i64,
    pub profile_identity_id: i64,
    pub scope: String,
}

/// Storage for `profile_identity_id` + `scope` auth pairs
pub struct Database {
    connection: Rc<RwLock<Connection>>,
}

impl Database {
    // Constructors

    /// Create new `Self`
    pub fn new(connection: &Rc<RwLock<Connection>>) -> Self {
        Self {
            connection: connection.clone(),
        }
    }

    // Actions

    /// Create new record in database
    pub fn add(&self, profile_identity_id: i64, scope: &str) -> Result<i64, Error> {
        // Begin new transaction
        let mut writable = self.connection.write().unwrap(); // @TODO
        let tx = writable.transaction()?;

        // Create new record
        insert(&tx, profile_identity_id, scope)?;

        // Hold insert ID for result
        let id = last_insert_id(&tx);

        // Done
        match tx.commit() {
            Ok(_) => Ok(id),
            Err(e) => Err(e),
        }
    }

    /// Delete record with given `id` from database
    pub fn delete(&self, id: i64) -> Result<(), Error> {
        // Begin new transaction
        let mut writable = self.connection.write().unwrap(); // @TODO
        let tx = writable.transaction()?;

        // Create new record
        delete(&tx, id)?;

        // Done
        match tx.commit() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    // Getters

    /// Get records from database match current `profile_id` optionally filtered by `scope`
    pub fn records_scope(&self, scope: Option<&str>) -> Result<Vec<Table>, Error> {
        let readable = self.connection.read().unwrap(); // @TODO
        let tx = readable.unchecked_transaction()?;
        select_scope(&tx, scope)
    }

    /// Get records from database match current `profile_id` optionally filtered by `scope`
    pub fn records_ref(&self, profile_identity_id: i64) -> Result<Vec<Table>, Error> {
        let readable = self.connection.read().unwrap(); // @TODO
        let tx = readable.unchecked_transaction()?;
        select_ref(&tx, profile_identity_id)
    }
}

// Low-level DB API

pub fn init(tx: &Transaction) -> Result<usize, Error> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_identity_auth`
        (
            `id`                  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_identity_id` INTEGER NOT NULL,
            `scope`               VARCHAR(1024) NOT NULL,

            FOREIGN KEY (`profile_identity_id`) REFERENCES `profile_identity`(`id`),
            UNIQUE (`scope`)
        )",
        [],
    )
}

pub fn insert(tx: &Transaction, profile_identity_id: i64, scope: &str) -> Result<usize, Error> {
    tx.execute(
        "INSERT INTO `profile_identity_auth` (
            `profile_identity_id`,
            `scope`
        ) VALUES (?, ?)",
        (profile_identity_id, scope),
    )
}

pub fn delete(tx: &Transaction, id: i64) -> Result<usize, Error> {
    tx.execute("DELETE FROM `profile_identity_auth` WHERE `id` = ?", [id])
}

pub fn select_scope(tx: &Transaction, scope: Option<&str>) -> Result<Vec<Table>, Error> {
    let mut stmt = tx.prepare(
        "SELECT `id`,
                `profile_identity_id`,
                `scope`

                FROM `profile_identity_auth`
                WHERE `scope` LIKE ?",
    )?;

    let result = stmt.query_map([scope.unwrap_or("%")], |row| {
        Ok(Table {
            id: row.get(0)?,
            profile_identity_id: row.get(1)?,
            scope: row.get(2)?,
        })
    })?;

    let mut records = Vec::new();

    for record in result {
        let table = record?;
        records.push(table);
    }

    Ok(records)
}

pub fn select_ref(tx: &Transaction, profile_identity_id: i64) -> Result<Vec<Table>, Error> {
    let mut stmt = tx.prepare(
        "SELECT `id`,
                `profile_identity_id`,
                `scope`

                FROM `profile_identity_auth`
                WHERE `profile_identity_id` = ?",
    )?;

    let result = stmt.query_map([profile_identity_id], |row| {
        Ok(Table {
            id: row.get(0)?,
            profile_identity_id: row.get(1)?,
            scope: row.get(2)?,
        })
    })?;

    let mut records = Vec::new();

    for record in result {
        let table = record?;
        records.push(table);
    }

    Ok(records)
}

pub fn last_insert_id(tx: &Transaction) -> i64 {
    tx.last_insert_rowid()
}
