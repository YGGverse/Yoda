use gtk::glib::DateTime;
use sqlite::{Connection, Error, Transaction};
use std::{rc::Rc, sync::RwLock};

pub struct Table {
    pub id: i64,
    //pub profile_id: i64,
    //pub time: DateTime,
    pub request: String,
}

pub struct Database {
    connection: Rc<RwLock<Connection>>,
    profile_id: Rc<i64>, // multi-profile relationship
}

impl Database {
    // Constructors

    /// Create new `Self`
    pub fn new(connection: &Rc<RwLock<Connection>>, profile_id: &Rc<i64>) -> Self {
        Self {
            connection: connection.clone(),
            profile_id: profile_id.clone(),
        }
    }

    // Getters

    /// Get bookmark records from database with optional filter by `request`
    pub fn records(&self, request: Option<&str>) -> Result<Vec<Table>, Error> {
        let readable = self.connection.read().unwrap(); // @TODO
        let tx = readable.unchecked_transaction()?;
        select(&tx, *self.profile_id, request)
    }

    // Setters

    /// Create new bookmark record in database
    /// * return last insert ID on success
    pub fn add(&self, time: DateTime, request: String) -> Result<i64, Error> {
        // Begin new transaction
        let mut writable = self.connection.write().unwrap(); // @TODO
        let tx = writable.transaction()?;

        // Create new record
        insert(&tx, *self.profile_id, time, request)?;

        // Hold insert ID for result
        let id = last_insert_id(&tx);

        // Done
        match tx.commit() {
            Ok(_) => Ok(id),
            Err(e) => Err(e),
        }
    }

    /// Delete bookmark record from database
    pub fn delete(&self, id: i64) -> Result<(), Error> {
        // Begin new transaction
        let mut writable = self.connection.write().unwrap(); // @TODO
        let tx = writable.transaction()?;

        // Delete record by ID
        match delete(&tx, id) {
            Ok(_) => match tx.commit() {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

// Low-level DB API

pub fn init(tx: &Transaction) -> Result<usize, Error> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_bookmark`
        (
            `id`         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_id` INTEGER NOT NULL,
            `time`       INTEGER NOT NULL,
            `request`    TEXT NOT NULL,

            FOREIGN KEY (`profile_id`) REFERENCES `profile`(`id`)
        )",
        [],
    )
}

pub fn insert(
    tx: &Transaction,
    profile_id: i64,
    time: DateTime,
    request: String,
) -> Result<usize, Error> {
    tx.execute(
        "INSERT INTO `profile_bookmark` (
            `profile_id`,
            `time`,
            `request`
        ) VALUES (?, ?, ?)",
        (profile_id, time.to_unix(), request),
    )
}

pub fn select(
    tx: &Transaction,
    profile_id: i64,
    request: Option<&str>,
) -> Result<Vec<Table>, Error> {
    let mut stmt = tx.prepare(
        "SELECT `id`, `profile_id`, `time`, `request`
            FROM `profile_bookmark`
            WHERE `profile_id` = ? AND `request` LIKE ?",
    )?;

    let result = stmt.query_map((profile_id, request.unwrap_or("%")), |row| {
        Ok(Table {
            id: row.get(0)?,
            //profile_id: row.get(1)?,
            //time: DateTime::from_unix_local(row.get(2)?).unwrap(),
            request: row.get(3)?,
        })
    })?;

    let mut records = Vec::new();

    for record in result {
        let table = record?;
        records.push(table);
    }

    Ok(records)
}

pub fn delete(tx: &Transaction, id: i64) -> Result<usize, Error> {
    tx.execute("DELETE FROM `profile_bookmark` WHERE `id` = ?", [id])
}

pub fn last_insert_id(tx: &Transaction) -> i64 {
    tx.last_insert_rowid()
}
