mod ignore;
mod rule;

use anyhow::Result;
use ignore::Ignore;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rule::Rule;
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

    pub fn rules(&self) -> Result<Vec<Rule>> {
        rules(&self.pool.get()?.unchecked_transaction()?, self.profile_id)
    }

    pub fn ignores(&self) -> Result<Vec<Ignore>> {
        ignores(&self.pool.get()?.unchecked_transaction()?, self.profile_id)
    }

    // Setters
}

// Low-level DB API

pub fn init(tx: &Transaction) -> Result<usize> {
    let mut s = 0;

    s += tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_proxy_ignore`
        (
            `id`         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_id` INTEGER NOT NULL,
            `time`       INTEGER NOT NULL,
            `is_enabled` INTEGER NOT NULL,
            `host`       VARCHAR(255) NOT NULL,

            FOREIGN KEY (`profile_id`) REFERENCES `profile` (`id`),
            UNIQUE (`host`)
        )",
        [],
    )?;

    s += tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_proxy_rule`
        (
            `id`         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_id` INTEGER NOT NULL,
            `time`       INTEGER NOT NULL,
            `is_enabled` INTEGER NOT NULL,
            `priority`   INTEGER NOT NULL,
            `regex`      VARCHAR(255) NOT NULL,
            `url`        VARCHAR(255) NOT NULL,

            FOREIGN KEY (`profile_id`) REFERENCES `profile` (`id`),
            UNIQUE (`regex`)
        )",
        [],
    )?;

    Ok(s)
}

pub fn ignores(tx: &Transaction, profile_id: i64) -> Result<Vec<Ignore>> {
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
        Ok(Ignore {
            //id: row.get(0)?,
            //profile_id: row.get(1)?,
            //time: DateTime::from_unix_local(row.get(2)?).unwrap(),
            host: row.get(3)?,
            is_enabled: row.get(4)?,
        })
    })?;

    let mut records = Vec::new();

    for record in result {
        let table = record?;
        records.push(table);
    }

    Ok(records)
}

pub fn rules(tx: &Transaction, profile_id: i64) -> Result<Vec<Rule>> {
    let mut stmt = tx.prepare(
        "SELECT `id`,
                `profile_id`,
                `time`,
                `is_enabled`,
                `priority`,
                `regex`,
                `url`

                FROM `profile_proxy_rule`
                WHERE `profile_id` = ?
                ORDER BY `priority` ASC",
    )?;

    let result = stmt.query_map([profile_id], |row| {
        Ok(Rule {
            //id: row.get(0)?,
            //profile_id: row.get(1)?,
            //time: DateTime::from_unix_local(row.get(2)?).unwrap(),
            is_enabled: row.get(3)?,
            //priority: row.get(4)?,
            regex: row.get(5)?,
            url: row.get(6)?,
        })
    })?;

    let mut records = Vec::new();

    for record in result {
        let table = record?;
        records.push(table);
    }

    Ok(records)
}
