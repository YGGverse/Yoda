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

    // Setters

    pub fn set(&self, (key, value): (String, String)) -> Result<i64> {
        let mut c = self.pool.get()?;
        let tx = c.transaction()?;
        let id = set(&tx, self.profile_id, key, value)?;
        tx.commit()?;
        Ok(id)
    }
}

// Low-level DB API

pub fn init(tx: &Transaction) -> Result<usize> {
    Ok(tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_proxy_misc`
        (
            `id`         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_id` INTEGER NOT NULL,
            `key`        VARCHAR(255) NOT NULL,
            `value`      TEXT NULL,

            FOREIGN KEY (`profile_id`) REFERENCES `profile` (`id`),
            UNIQUE (`key`)
        )",
        [],
    )?)
}

fn set(tx: &Transaction, profile_id: i64, key: String, value: String) -> Result<i64> {
    tx.execute(
        "INSERT INTO `profile_proxy_misc` (
            `profile_id`,
            `key`,
            `value`
        ) VALUES (?, ?, ?) ON CONFLICT (`key`) DO UPDATE SET `value` = `excluded`.`value`",
        (profile_id, key, value),
    )?;
    Ok(tx.last_insert_rowid())
}

fn rows(tx: &Transaction, profile_id: i64) -> Result<Vec<Row>> {
    let mut stmt = tx.prepare(
        "SELECT `id`,
                `profile_id`,
                `key`,
                `value`

                FROM `profile_proxy_misc`
                WHERE `profile_id` = ?",
    )?;

    let result = stmt.query_map([profile_id], |row| {
        Ok(Row {
            //id: row.get(0)?,
            //profile_id: row.get(1)?,
            key: row.get(2)?,
            value: row.get(3)?,
        })
    })?;

    let mut rows = Vec::new();

    for r in result {
        let row = r?;
        rows.push(row);
    }

    Ok(rows)
}
