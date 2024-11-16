use std::{rc::Rc, sync::RwLock};

use sqlite::{Connection, Error, Transaction};

pub struct Table {
    //pub id: i64,
    //pub profile_id: i64,
    pub gemini_id: i64,
}

/// Storage for `gemini_id` + `request` auth pairs
pub struct Database {
    connection: Rc<RwLock<Connection>>,
    profile_id: Rc<i64>, // multi-profile relationship
}

impl Database {
    // Constructors

    /// Create new `Self`
    pub fn new(connection: Rc<RwLock<Connection>>, profile_id: Rc<i64>) -> Self {
        Self {
            connection,
            profile_id,
        }
    }

    // Getters

    /// Get records from database match current `profile_id` optionally filtered by `request`
    pub fn records(&self, request: Option<&str>) -> Result<Vec<Table>, Error> {
        let readable = self.connection.read().unwrap(); // @TODO
        let tx = readable.unchecked_transaction()?;
        select(&tx, *self.profile_id, request)
    }
}

// Low-level DB API

pub fn init(tx: &Transaction) -> Result<usize, Error> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_identity_gemini_auth`
        (
            `id`         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_id` INTEGER NOT NULL,
            `gemini_id`  INTEGER NOT NULL,
            `request`    TEXT NOT NULL
        )",
        [],
    )
}

pub fn select(
    tx: &Transaction,
    profile_id: i64,
    request: Option<&str>,
) -> Result<Vec<Table>, Error> {
    let mut stmt = tx.prepare(
        "SELECT `id`,
                `profile_id`,
                `gemini_id` FROM `profile_identity_gemini_auth`
                            WHERE `profile_id` = ? AND `request` LIKE ?",
    )?;

    let result = stmt.query_map((profile_id, request.unwrap_or("%")), |row| {
        Ok(Table {
            //id: row.get(0)?,
            //profile_id: row.get(1)?,
            gemini_id: row.get(2)?,
        })
    })?;

    let mut records = Vec::new();

    for record in result {
        let table = record?;
        records.push(table);
    }

    Ok(records)
}
