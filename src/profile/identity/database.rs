use sqlite::{Connection, Error, Transaction};
use std::{rc::Rc, sync::RwLock};

pub struct Table {
    pub id: i64,
    pub profile_id: i64,
    pub is_active: bool,
}

pub struct Database {
    pub connection: Rc<RwLock<Connection>>,
}

impl Database {
    // Constructors

    /// Create new `Self`
    pub fn new(connection: Rc<RwLock<Connection>>) -> Self {
        Self { connection }
    }

    // Getters

    /// Get all records
    pub fn records(&self) -> Result<Vec<Table>, Error> {
        let readable = self.connection.read().unwrap();
        let tx = readable.unchecked_transaction()?;
        select(&tx)
    }

    /// Get active identity record if exist
    pub fn active(&self) -> Result<Option<Table>, Error> {
        let records = self.records()?;
        Ok(records.into_iter().find(|record| record.is_active))
    }

    // Setters

    /// Create new record in `Self` database connected
    pub fn add(&self, profile_id: Rc<i64>, is_active: bool) -> Result<i64, Error> {
        // Begin new transaction
        let mut writable = self.connection.write().unwrap();
        let tx = writable.transaction()?;

        // New record has active status
        if is_active {
            // Deactivate other records as only one profile should be active
            for record in select(&tx)? {
                update(&tx, record.profile_id, record.id, false)?;
            }
        }

        // Create new record
        insert(&tx, profile_id, is_active)?;

        // Hold insert ID for result
        let id = last_insert_id(&tx);

        // Done
        match tx.commit() {
            Ok(_) => Ok(id),
            Err(e) => Err(e),
        }
    }
}

// Low-level DB API

pub fn init(tx: &Transaction) -> Result<usize, Error> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_identity`
        (
            `id`         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_id` INTEGER NOT NULL,
            `is_active`  INTEGER NOT NULL,

            FOREIGN KEY (`profile_id`) REFERENCES `profile`(`id`)
        )",
        [],
    )
}

pub fn insert(tx: &Transaction, profile_id: Rc<i64>, is_active: bool) -> Result<usize, Error> {
    tx.execute(
        "INSERT INTO `profile_identity` (
            `profile_id`,
            `is_active`
        ) VALUES (?, ?)",
        (profile_id, is_active),
    )
}

pub fn update(tx: &Transaction, id: i64, profile_id: i64, is_active: bool) -> Result<usize, Error> {
    tx.execute(
        "UPDATE `profile_identity`
        SET `profile_id` = ?,
            `is_active`  = ?
        WHERE
            `id` = ?",
        (profile_id, is_active, id),
    )
}

pub fn select(tx: &Transaction) -> Result<Vec<Table>, Error> {
    let mut stmt = tx.prepare("SELECT `id`, `profile_id`, `is_active` FROM `profile_identity`")?;
    let result = stmt.query_map([], |row| {
        Ok(Table {
            id: row.get(0)?,
            profile_id: row.get(1)?,
            is_active: row.get(2)?,
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
