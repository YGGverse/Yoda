use sqlite::{Connection, Error, Transaction};
use std::{rc::Rc, sync::RwLock};

pub const NAME_MAX_LEN: i32 = 36;

pub struct Table {
    pub id: i64,
    //pub profile_identity_id: i64,
    pub pem: String,
    pub name: Option<String>,
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
        format!(
            "CREATE TABLE IF NOT EXISTS `profile_identity_gemini`
            (
                `id`                  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `profile_identity_id` INTEGER NOT NULL,
                `pem`                 TEXT NOT NULL,
                `name`                VARCHAR({}),

                FOREIGN KEY (`profile_identity_id`) REFERENCES `profile_identity`(`id`)
            )",
            NAME_MAX_LEN
        )
        .as_str(),
        [],
    )
}

pub fn select(tx: &Transaction, profile_identity_id: i64) -> Result<Vec<Table>, Error> {
    let mut stmt = tx.prepare(
        "SELECT `id`,
                `profile_identity_id`,
                `pem`,
                `name`

        FROM `profile_identity_gemini` WHERE `profile_identity_id` = ?",
    )?;

    let result = stmt.query_map([profile_identity_id], |row| {
        Ok(Table {
            id: row.get(0)?,
            //profile_identity_id: row.get(1)?,
            pem: row.get(2)?,
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
