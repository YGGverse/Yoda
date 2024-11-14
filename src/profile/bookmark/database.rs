use gtk::glib::DateTime;
use sqlite::{Connection, Error, Transaction};
use std::{rc::Rc, sync::RwLock};

pub struct Table {
    pub id: i64,
    pub profile_id: i64,
    pub time: DateTime,
    pub request: String,
}

pub struct Database {
    pub connection: Rc<RwLock<Connection>>,
    profile_id: i64, // multi-profile relationship @TODO
}

impl Database {
    // Constructors

    pub fn new(connection: Rc<RwLock<Connection>>, profile_id: i64) -> Self {
        Self {
            connection,
            profile_id,
        }
    }

    // Getters

    pub fn records(&self, request: Option<&str>) -> Vec<Table> {
        let readable = self.connection.read().unwrap();
        let tx = readable.unchecked_transaction().unwrap();
        select(&tx, self.profile_id, request).unwrap()
    }

    // Setters

    pub fn add(&self, time: DateTime, request: String) -> Result<i64, ()> {
        // Begin new transaction
        let mut writable = self.connection.write().unwrap();
        let tx = writable.transaction().unwrap();

        // Create new record
        insert(&tx, self.profile_id, time, request).unwrap();

        // Hold insert ID for result
        let id = last_insert_id(&tx);

        // Done
        match tx.commit() {
            Ok(_) => Ok(id),
            Err(_) => Err(()), // @TODO
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
            `request`    TEXT NOT NULL
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

    let filter = match request {
        Some(value) => value,
        None => "%",
    };

    let result = stmt.query_map((profile_id, filter), |row| {
        Ok(Table {
            id: row.get(0)?,
            profile_id: row.get(1)?,
            time: DateTime::from_unix_local(row.get(2)?).unwrap(),
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
