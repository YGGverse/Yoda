use sqlite::{Connection, Error, Transaction};
use std::{rc::Rc, sync::RwLock};

pub struct Table {
    pub id: i64,
    //pub profile_identity_id: i64,
    pub pem: String,
}

/// Storage for Gemini auth certificates
pub struct Database {
    connection: Rc<RwLock<Connection>>,
    profile_identity_id: Rc<i64>, // multi-profile relationship
}

impl Database {
    // Constructors

    /// Create new `Self`
    pub fn new(connection: Rc<RwLock<Connection>>, profile_identity_id: Rc<i64>) -> Self {
        Self {
            connection,
            profile_identity_id,
        }
    }

    // Actions

    /// Create new record in database
    pub fn add(&self, pem: &str) -> Result<i64, Error> {
        // Begin new transaction
        let mut writable = self.connection.write().unwrap(); // @TODO
        let tx = writable.transaction()?;

        // Create new record
        insert(&tx, *self.profile_identity_id, pem)?;

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

    /// Get single record match `id`
    pub fn record(&self, id: i64) -> Result<Option<Table>, Error> {
        let readable = self.connection.read().unwrap();
        let tx = readable.unchecked_transaction()?;
        let records = select(&tx, *self.profile_identity_id)?; // @TODO single record query

        for record in records {
            if record.id == id {
                return Ok(Some(record));
            }
        }

        Ok(None)
    }

    /// Get all records match current `profile_identity_id`
    pub fn records(&self) -> Result<Vec<Table>, Error> {
        let readable = self.connection.read().unwrap(); // @TODO
        let tx = readable.unchecked_transaction()?;
        select(&tx, *self.profile_identity_id)
    }
}

// Low-level DB API

pub fn init(tx: &Transaction) -> Result<usize, Error> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_identity_gemini`
        (
            `id`                  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_identity_id` INTEGER NOT NULL,
            `pem`                 TEXT NOT NULL,

            FOREIGN KEY (`profile_identity_id`) REFERENCES `profile_identity`(`id`)
        )",
        [],
    )
}

pub fn insert(tx: &Transaction, profile_identity_id: i64, pem: &str) -> Result<usize, Error> {
    tx.execute(
        "INSERT INTO `profile_identity_gemini` (
            `profile_identity_id`,
            `pem`
        ) VALUES (?, ?)",
        (profile_identity_id, pem),
    )
}

pub fn delete(tx: &Transaction, id: i64) -> Result<usize, Error> {
    tx.execute("DELETE FROM `profile_identity_gemini` WHERE `id` = ?", [id])
}

pub fn select(tx: &Transaction, profile_identity_id: i64) -> Result<Vec<Table>, Error> {
    let mut stmt = tx.prepare(
        "SELECT `id`,
                `profile_identity_id`,
                `pem`

        FROM `profile_identity_gemini` WHERE `profile_identity_id` = ?",
    )?;

    let result = stmt.query_map([profile_identity_id], |row| {
        Ok(Table {
            id: row.get(0)?,
            //profile_identity_id: row.get(1)?,
            pem: row.get(2)?,
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
