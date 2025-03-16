use super::{Item, item::Event};
use anyhow::Result;
use gtk::glib::DateTime;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use sqlite::Transaction;

pub struct Database {
    pool: Pool<SqliteConnectionManager>,
    profile_id: i64,
}

impl Database {
    // Constructors

    /// Create new `Self`
    pub fn build(pool: &Pool<SqliteConnectionManager>, profile_id: i64) -> Self {
        Self {
            pool: pool.clone(),
            profile_id,
        }
    }

    // Getters

    /// Get history records from database with optional filter by `request`
    pub fn records(&self, request: Option<&str>, title: Option<&str>) -> Result<Vec<Item>> {
        select(
            &self.pool.get()?.unchecked_transaction()?,
            self.profile_id,
            request,
            title,
        )
    }

    // Actions

    /// Create new history record in database
    /// * return last insert ID on success
    pub fn add(&self, item: &Item) -> Result<i64> {
        let mut connection = self.pool.get()?;
        let tx = connection.transaction()?;
        let id = insert(&tx, self.profile_id, item)?;
        tx.commit()?;
        Ok(id)
    }

    pub fn update(&self, item: &Item) -> Result<usize> {
        let mut connection = self.pool.get()?;
        let tx = connection.transaction()?;
        let affected = update(&tx, self.profile_id, item)?;
        tx.commit()?;
        Ok(affected)
    }
}

// Low-level DB API

pub fn init(tx: &Transaction) -> Result<usize> {
    Ok(tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_history`
        (
            `id`           INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_id`   INTEGER NOT NULL,
            `opened_time`  INTEGER NOT NULL,
            `opened_count` INTEGER NOT NULL,
            `closed_time`  INTEGER NULL,
            `closed_count` INTEGER NULL,
            `request`      TEXT NOT NULL,
            `title`        TEXT NULL,

            FOREIGN KEY (`profile_id`) REFERENCES `profile` (`id`),
            UNIQUE (`profile_id`, `request`)
        )",
        [],
    )?)
}

pub fn insert(tx: &Transaction, profile_id: i64, item: &Item) -> Result<i64> {
    tx.execute(
        "INSERT INTO `profile_history` (
            `profile_id`,
            `opened_time`,
            `opened_count`,
            `closed_time`,
            `closed_count`,
            `request`,
            `title`
        ) VALUES (?, ?, ?, ?, ?, ?, ?)",
        (
            profile_id,
            item.opened.time.to_unix(),
            item.opened.count as i64,
            item.closed.as_ref().map(|closed| closed.time.to_unix()),
            item.closed.as_ref().map(|closed| closed.count as i64),
            item.request.as_str(),
            item.title.as_deref(),
        ),
    )?;
    Ok(tx.last_insert_rowid())
}

pub fn update(tx: &Transaction, profile_id: i64, item: &Item) -> Result<usize> {
    Ok(tx.execute(
        "UPDATE `profile_history`
            SET `opened_time` = ?,
                `opened_count` = ?,
                `closed_time` = ?,
                `closed_count` = ?,
                `request` = ?,
                `title` = ?
        WHERE `id` = ? AND `profile_id` = ?",
        (
            item.opened.time.to_unix(),
            item.opened.count as i64,
            item.closed.as_ref().map(|closed| closed.time.to_unix()),
            item.closed.as_ref().map(|closed| closed.count as i64),
            item.request.as_str(),
            item.title.as_deref(),
            item.id.unwrap(),
            profile_id,
        ),
    )?)
}

pub fn select(
    tx: &Transaction,
    profile_id: i64,
    request: Option<&str>,
    title: Option<&str>,
) -> Result<Vec<Item>> {
    let mut stmt = tx.prepare(
        "SELECT
            `id`,
            `profile_id`,
            `opened_time`,
            `opened_count`,
            `closed_time`,
            `closed_count`,
            `request`,
            `title`
        FROM `profile_history`
        WHERE `profile_id` = ? AND (`request` LIKE ? OR `title` LIKE ?)",
    )?;

    let result = stmt.query_map(
        (profile_id, request.unwrap_or("%"), title.unwrap_or("%")),
        |row| {
            Ok(Item {
                id: row.get(0)?,
                //profile_id: row.get(1)?,
                opened: Event {
                    time: DateTime::from_unix_local(row.get(2)?).unwrap(),
                    count: row.get(3)?,
                },
                closed: closed(row.get(4)?, row.get(5)?),
                request: row.get::<_, String>(6)?.into(),
                title: row.get::<_, Option<String>>(7)?.map(|s| s.into()),
                is_saved: true,
            })
        },
    )?;

    let mut items = Vec::new();

    for record in result {
        let item = record?;
        items.push(item);
    }

    Ok(items)
}

// Tools

fn closed(time: Option<i64>, count: Option<i64>) -> Option<Event> {
    if let Some(t) = time {
        if let Some(c) = count {
            return Some(Event {
                time: DateTime::from_unix_local(t).unwrap(),
                count: c as usize,
            });
        }
        panic!()
    }
    None
}
