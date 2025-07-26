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

    pub fn clean(&self, keep_id: Vec<i64>) -> Result<()> {
        let mut c = self.pool.get()?;
        let tx = c.transaction()?;
        clean(&tx, keep_id)?;
        tx.commit()?;
        Ok(())
    }

    pub fn persist(
        &self,
        id: Option<i64>,
        time: i64,
        is_enabled: bool,
        priority: i32,
        request: String,
        url: String,
    ) -> Result<i64> {
        let mut c = self.pool.get()?;
        let tx = c.transaction()?;
        let id = match id {
            Some(id) => {
                update(&tx, id, time, is_enabled, priority, request, url)?;
                id
            }
            None => insert(
                &tx,
                self.profile_id,
                time,
                is_enabled,
                priority,
                request,
                url,
            )?,
        };
        tx.commit()?;
        Ok(id)
    }
}

// Low-level DB API

pub fn init(tx: &Transaction) -> Result<usize> {
    Ok(tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_proxy_rule`
        (
            `id`         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_id` INTEGER NOT NULL,
            `time`       INTEGER NOT NULL,
            `is_enabled` INTEGER NOT NULL,
            `priority`   INTEGER NOT NULL,
            `request`    VARCHAR(1024) NOT NULL,
            `url`        VARCHAR(255) NOT NULL,

            FOREIGN KEY (`profile_id`) REFERENCES `profile` (`id`)
        )",
        [],
    )?)
}

fn clean(tx: &Transaction, keep_id: Vec<i64>) -> Result<usize> {
    if keep_id.is_empty() {
        return Ok(0);
    }
    Ok(tx.execute(
        &format!(
            "DELETE FROM `profile_proxy_rule` WHERE `id` NOT IN ({})",
            keep_id
                .into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<String>>()
                .join(",")
        ),
        [],
    )?)
}

fn insert(
    tx: &Transaction,
    profile_id: i64,
    time: i64,
    is_enabled: bool,
    priority: i32,
    request: String,
    url: String,
) -> Result<i64> {
    tx.execute(
        "INSERT INTO `profile_proxy_rule` (
            `profile_id`,
            `time`,
            `is_enabled`,
            `priority`,
            `request`,
            `url`
        ) VALUES (?, ?, ?, ?, ?, ?)",
        (profile_id, time, is_enabled, priority, request, url),
    )?;
    Ok(tx.last_insert_rowid())
}

fn update(
    tx: &Transaction,
    id: i64,
    time: i64,
    is_enabled: bool,
    priority: i32,
    request: String,
    url: String,
) -> Result<usize> {
    Ok(tx.execute(
        "UPDATE `profile_proxy_rule`
            SET `time` = ?,
                `is_enabled` = ?,
                `priority` = ?,
                `request` = ?,
                `url` = ?

            WHERE `id` = ?",
        (time, is_enabled, priority, request, url, id),
    )?)
}

fn rows(tx: &Transaction, profile_id: i64) -> Result<Vec<Row>> {
    let mut stmt = tx.prepare(
        "SELECT `id`,
                `profile_id`,
                `time`,
                `is_enabled`,
                `priority`,
                `request`,
                `url`

                FROM `profile_proxy_rule`
                WHERE `profile_id` = ?
                ORDER BY `priority` ASC",
    )?;

    let result = stmt.query_map([profile_id], |row| {
        Ok(Row {
            id: row.get(0)?,
            //profile_id: row.get(1)?,
            time: row.get(2)?,
            is_enabled: row.get(3)?,
            priority: row.get(4)?,
            request: row.get(5)?,
            url: row.get(6)?,
        })
    })?;

    let mut rows = Vec::new();

    for r in result {
        rows.push(r?);
    }

    Ok(rows)
}
