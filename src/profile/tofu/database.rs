mod row;

use anyhow::Result;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use row::Row;
use sqlite::Transaction;

pub struct Database {
    pool: Pool<SqliteConnectionManager>,
    profile_id: i64,
}

impl Database {
    // Constructors

    /// Create new `Self`
    pub fn init(pool: &Pool<SqliteConnectionManager>, profile_id: i64) -> Self {
        Self {
            pool: pool.clone(),
            profile_id,
        }
    }

    // Getters

    /// Get records from database with optional filter by `request`
    pub fn records(&self) -> Result<Vec<Row>> {
        select(&self.pool.get()?.unchecked_transaction()?, self.profile_id)
    }

    // Setters

    /// Create new record in database
    /// * return last insert ID on success
    pub fn add(&self, host: String, port: i32, time: i64, pem: &str) -> Result<i64> {
        let mut connection = self.pool.get()?;
        let tx = connection.transaction()?;
        let id = insert(&tx, self.profile_id, host, port, time, pem)?;
        tx.commit()?;
        Ok(id)
    }
}

// Low-level DB API

pub fn init(tx: &Transaction) -> Result<usize> {
    Ok(tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_tofu`
        (
            `id`         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_id` INTEGER NOT NULL,
            `time`       INTEGER NOT NULL,
            `port`       INTEGER NOT NULL,
            `host`       VARCHAR(1024) NOT NULL,
            `pem`        TEXT NOT NULL,

            FOREIGN KEY (`profile_id`) REFERENCES `profile` (`id`),
            UNIQUE (`host`, `port`)
        )",
        [],
    )?)
}

pub fn insert(
    tx: &Transaction,
    profile_id: i64,
    host: String,
    port: i32,
    time: i64,
    pem: &str,
) -> Result<i64> {
    tx.execute(
        "INSERT INTO `profile_tofu` (
            `profile_id`,
            `time`,
            `host`,
            `port`,
            `pem`
        ) VALUES (?, ?, ?, ?, ?) ON CONFLICT (`host`, `port`)
                                 DO UPDATE SET `time` = `excluded`.`time`,
                                               `pem`  = `excluded`.`pem`",
        (profile_id, time, host, port, pem),
    )?;
    Ok(tx.last_insert_rowid())
}

pub fn select(tx: &Transaction, profile_id: i64) -> Result<Vec<Row>> {
    let mut stmt = tx.prepare(
        "SELECT `id`,
                `profile_id`,
                `host`,
                `port`,
                `time`,
                `pem` FROM `profile_tofu` WHERE `profile_id` = ?",
    )?;

    let result = stmt.query_map([profile_id], |row| {
        Ok(Row {
            id: row.get(0)?,
            //profile_id: row.get(1)?,
            host: row.get(2)?,
            port: row.get(3)?,
            time: row.get(4)?,
            pem: row.get(5)?,
        })
    })?;

    let mut records = Vec::new();

    for record in result {
        let table = record?;
        records.push(table);
    }

    Ok(records)
}
