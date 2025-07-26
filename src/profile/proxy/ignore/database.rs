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

    pub fn init(pool: &Pool<SqliteConnectionManager>, profile_id: i64) -> Self {
        Self {
            pool: pool.clone(),
            profile_id,
        }
    }

    // Getters

    pub fn rows(&self) -> Result<Vec<Row>> {
        rows(&self.pool.get()?.unchecked_transaction()?, self.profile_id)
    }
}

// Low-level DB API

pub fn init(tx: &Transaction) -> Result<usize> {
    Ok(tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_proxy_ignore`
        (
            `id`         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_id` INTEGER NOT NULL,
            `time`       INTEGER NOT NULL,
            `is_enabled` INTEGER NOT NULL,
            `host`       VARCHAR(255) NOT NULL,

            FOREIGN KEY (`profile_id`) REFERENCES `profile` (`id`)
        )",
        [],
    )?)
}

fn rows(tx: &Transaction, profile_id: i64) -> Result<Vec<Row>> {
    let mut stmt = tx.prepare(
        "SELECT `id`,
                `profile_id`,
                `time`,
                `host`,
                `is_enabled`

                FROM `profile_proxy_ignore`
                WHERE `profile_id` = ?",
    )?;

    let result = stmt.query_map([profile_id], |row| {
        Ok(Row {
            //id: row.get(0)?,
            //profile_id: row.get(1)?,
            //time: DateTime::from_unix_local(row.get(2)?).unwrap(),
            host: row.get(3)?,
            is_enabled: row.get(4)?,
        })
    })?;

    let mut rows = Vec::new();

    for r in result {
        let row = r?;
        rows.push(row);
    }

    Ok(rows)
}
