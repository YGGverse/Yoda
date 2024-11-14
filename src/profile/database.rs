use gtk::glib::DateTime;
use sqlite::{Connection, Error, Transaction};
use std::{rc::Rc, sync::RwLock};

pub struct Table {
    pub id: i64,
    pub is_active: bool,
    pub time: DateTime,
    pub name: Option<String>,
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
    pub fn records(&self) -> Vec<Table> {
        let readable = self.connection.read().unwrap();
        let tx = readable.unchecked_transaction().unwrap();
        select(&tx).unwrap()
    }

    /// Get active profile record if exist
    pub fn active(&self) -> Option<Table> {
        self.records().into_iter().find(|record| record.is_active)
    }

    // Setters

    /// Create new record in `Self` database connected
    pub fn add(&self, is_active: bool, time: DateTime, name: Option<String>) -> Result<i64, ()> {
        // Begin new transaction
        let mut writable = self.connection.write().unwrap();
        let tx = writable.transaction().unwrap();

        // New record has active status
        if is_active {
            // Deactivate other records as only one profile should be active
            for record in select(&tx).unwrap() {
                let _ = update(&tx, record.id, false, record.time, record.name);
            }
        }

        // Create new record
        insert(&tx, is_active, time, name).unwrap();

        // Hold insert ID for result
        let id = last_insert_id(&tx);

        // Done
        match tx.commit() {
            Ok(_) => Ok(id),
            Err(_) => Err(()), // @TODO
        }
    }

    /* @TODO not in use
    /// Set `is_active` status `true` for the record with given profile ID
    /// * reset other records to `false`
    pub fn activate(&self, id: i64) -> Result<(), ()> {
        // Begin new transaction
        let mut writable = self.connection.write().unwrap();
        let tx = writable.transaction().unwrap();

        // Deactivate other records as only one profile should be active
        for record in select(&tx).unwrap() {
            let _ = update(
                &tx,
                record.id,
                if record.id == id { true } else { false },
                record.time,
                record.name,
            );
        }

        // Done
        match tx.commit() {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        } // @TODO make sure ID exist and was changed
    } */
}

// Low-level DB API

pub fn init(tx: &Transaction) -> Result<usize, Error> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile`
        (
            `id`        INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `is_active` INTEGER NOT NULL,
            `time`      INTEGER NOT NULL,
            `name`      VARCHAR(255)
        )",
        [],
    )
}

pub fn insert(
    tx: &Transaction,
    is_active: bool,
    time: DateTime,
    name: Option<String>,
) -> Result<usize, Error> {
    tx.execute(
        "INSERT INTO `profile` (
            `is_active`,
            `time`,
            `name`
        ) VALUES (?, ?, ?)",
        (is_active, time.to_unix(), name),
    )
}

pub fn update(
    tx: &Transaction,
    id: i64,
    is_active: bool,
    time: DateTime,
    name: Option<String>,
) -> Result<usize, Error> {
    tx.execute(
        "UPDATE `profile` SET `is_active` = ?, `time` = ?, `name` = ? WHERE `id` = ?",
        (is_active, time.to_unix(), name, id),
    )
}

pub fn select(tx: &Transaction) -> Result<Vec<Table>, Error> {
    let mut stmt = tx.prepare("SELECT `id`, `is_active`, `time`, `name` FROM `profile`")?;
    let result = stmt.query_map([], |row| {
        Ok(Table {
            id: row.get(0)?,
            is_active: row.get(1)?,
            time: DateTime::from_unix_local(row.get(2)?).unwrap(),
            name: row.get(3)?,
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
