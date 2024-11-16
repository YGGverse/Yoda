use std::{rc::Rc, sync::RwLock};

use sqlite::{Connection, Error, Transaction};

pub struct Table {
    //pub id: i64,
    pub profile_identity_gemini_id: i64,
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
            `is_active`                  INTEGER NOT NULL,
            `url`                        VARCHAR(1024) NOT NULL,

            FOREIGN KEY (`profile_identity_gemini_id`) REFERENCES `profile_identity_gemini`(`id`),

            UNIQUE (
                `profile_identity_gemini_id`,
                `is_active`,
                `url`
            )
        )",
        [],
    )
}

pub fn select(tx: &Transaction, url: Option<&str>) -> Result<Vec<Table>, Error> {
    let mut stmt = tx.prepare(
        "SELECT `id`,
                `profile_identity_gemini_id`

                FROM `profile_identity_gemini_auth`
                WHERE `url` LIKE ?",
    )?;

    let result = stmt.query_map([url.unwrap_or("%")], |row| {
        Ok(Table {
            //id: row.get(0)?,
            profile_identity_gemini_id: row.get(1)?,
        })
    })?;

    let mut records = Vec::new();

    for record in result {
        let table = record?;
        records.push(table);
    }

    Ok(records)
}
