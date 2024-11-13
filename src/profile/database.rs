use gtk::glib::DateTime;
use sqlite::{Connection, Error, Transaction};
use std::{rc::Rc, sync::RwLock};

#[derive(Debug)]
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
        let binding = self.connection.read().unwrap();
        let tx = binding.unchecked_transaction().unwrap();

        records(&tx).unwrap()
    }

    /// Get selected profile record if exist
    pub fn selected(&self) -> Option<Table> {
        let binding = self.connection.read().unwrap();
        let tx = binding.unchecked_transaction().unwrap();

        for record in records(&tx).unwrap() {
            if record.is_active {
                return Some(record);
            }
        }

        None
    }
}

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

pub fn add(
    tx: &Transaction,
    is_active: bool,
    time: &DateTime,
    name: Option<&str>,
) -> Result<usize, Error> {
    tx.execute("INSERT INTO `profile`", (is_active, time.to_unix(), name))
}

pub fn records(tx: &Transaction) -> Result<Vec<Table>, Error> {
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

pub fn delete(tx: &Transaction, id: &i64) -> Result<usize, Error> {
    tx.execute("DELETE FROM `profile` WHERE `id` = ?", [id])
}

pub fn last_insert_id(tx: &Transaction) -> i64 {
    tx.last_insert_rowid()
}
