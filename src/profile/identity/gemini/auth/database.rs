use sqlite::{Connection, Error, Transaction};
use std::{rc::Rc, sync::RwLock};

pub struct Table {
    pub id: i64,
    pub profile_identity_gemini_id: i64,
    pub url: String,
}

/// Storage for `profile_identity_gemini_id` + `url` auth pairs
pub struct Database {
    connection: Rc<RwLock<Connection>>,
}

impl Database {
    // Constructors

    /// Create new `Self`
    pub fn new(connection: Rc<RwLock<Connection>>) -> Self {
        Self { connection }
    }

    // Actions

    /// Create new record in database
    pub fn add(&self, profile_identity_gemini_id: i64, url: &str) -> Result<i64, Error> {
        // Begin new transaction
        let mut writable = self.connection.write().unwrap(); // @TODO
        let tx = writable.transaction()?;

        // Create new record
        insert(&tx, profile_identity_gemini_id, url)?;

        // Hold insert ID for result
        let id = last_insert_id(&tx);

        // Done
        match tx.commit() {
            Ok(_) => Ok(id),
            Err(reason) => Err(reason),
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
            Err(reason) => Err(reason),
        }
    }

    // Getters

    /// Get records from database match current `profile_id` optionally filtered by `url`
    pub fn records(&self, url: Option<&str>) -> Result<Vec<Table>, Error> {
        let readable = self.connection.read().unwrap(); // @TODO
        let tx = readable.unchecked_transaction()?;
        select(&tx, url)
    }
}

// Low-level DB API

pub fn init(tx: &Transaction) -> Result<usize, Error> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_identity_gemini_auth`
        (
            `id`                         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_identity_gemini_id` INTEGER NOT NULL,
            `url`                        VARCHAR(1024) NOT NULL,

            FOREIGN KEY (`profile_identity_gemini_id`) REFERENCES `profile_identity_gemini`(`id`),
            UNIQUE (`url`)
        )",
        [],
    )
}

pub fn insert(
    tx: &Transaction,
    profile_identity_gemini_id: i64,
    url: &str,
) -> Result<usize, Error> {
    tx.execute(
        "INSERT INTO `profile_identity_gemini_auth` (
            `profile_identity_gemini_id`,
            `url`
        ) VALUES (?, ?)",
        (profile_identity_gemini_id, url),
    )
}

pub fn delete(tx: &Transaction, id: i64) -> Result<usize, Error> {
    tx.execute(
        "DELETE FROM `profile_identity_gemini_auth` WHERE `id` = ?",
        [id],
    )
}

pub fn select(tx: &Transaction, url: Option<&str>) -> Result<Vec<Table>, Error> {
    let mut stmt = tx.prepare(
        "SELECT `id`,
                `profile_identity_gemini_id`,
                `url`

                FROM `profile_identity_gemini_auth`
                WHERE `url` LIKE ?",
    )?;

    let result = stmt.query_map([url.unwrap_or("%")], |row| {
        Ok(Table {
            id: row.get(0)?,
            profile_identity_gemini_id: row.get(1)?,
            url: row.get(2)?,
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
