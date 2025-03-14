use super::Item;
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
    pub fn new(pool: &Pool<SqliteConnectionManager>, profile_id: i64) -> Self {
        Self {
            pool: pool.clone(),
            profile_id,
        }
    }

    // Getters

    /// Get bookmark records from database with optional filter by `request`
    pub fn records(&self, request: Option<&str>, title: Option<&str>) -> Result<Vec<Item>> {
        select(
            &self.pool.get()?.unchecked_transaction()?,
            self.profile_id,
            request,
            title,
        )
    }

    // Setters

    /// Create new bookmark record in database
    /// * return last insert ID on success
    pub fn add(&self, time: DateTime, request: &str, title: Option<&str>) -> Result<i64> {
        let mut connection = self.pool.get()?;
        let tx = connection.transaction()?;
        let id = insert(&tx, self.profile_id, time, request, title)?;
        tx.commit()?;
        Ok(id)
    }

    /// Delete bookmark record from database
    pub fn delete(&self, id: i64) -> Result<usize> {
        let mut connection = self.pool.get()?;
        let tx = connection.transaction()?;
        let usize = delete(&tx, id)?;
        tx.commit()?;
        Ok(usize)
    }
}

// Low-level DB API

pub fn init(tx: &Transaction) -> Result<usize> {
    Ok(tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_bookmark`
        (
            `id`         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_id` INTEGER NOT NULL,
            `time`       INTEGER NOT NULL,
            `request`    TEXT NOT NULL,
            `title`      TEXT NULL,

            FOREIGN KEY (`profile_id`) REFERENCES `profile` (`id`),
            UNIQUE (`profile_id`, `request`)
        )",
        [],
    )?)
}

pub fn insert(
    tx: &Transaction,
    profile_id: i64,
    time: DateTime,
    request: &str,
    title: Option<&str>,
) -> Result<i64> {
    tx.execute(
        "INSERT INTO `profile_bookmark` (
            `profile_id`,
            `time`,
            `request`,
            `title`
        ) VALUES (?, ?, ?, ?)",
        (profile_id, time.to_unix(), request, title),
    )?;
    Ok(tx.last_insert_rowid())
}

pub fn select(
    tx: &Transaction,
    profile_id: i64,
    request: Option<&str>,
    title: Option<&str>,
) -> Result<Vec<Item>> {
    let mut stmt = tx.prepare(
        "SELECT `id`, `profile_id`, `time`, `request`, `title`
            FROM `profile_bookmark`
            WHERE `profile_id` = ? AND (`request` LIKE ? OR `title` LIKE ?)",
    )?;

    let result = stmt.query_map(
        (profile_id, request.unwrap_or("%"), title.unwrap_or("%")),
        |row| {
            Ok(Item {
                id: row.get(0)?,
                //profile_id: row.get(1)?,
                //time: DateTime::from_unix_local(row.get(2)?).unwrap(),
                request: row.get(3)?,
                title: row.get(4)?,
            })
        },
    )?;

    let mut records = Vec::new();

    for record in result {
        let table = record?;
        records.push(table);
    }

    Ok(records)
}

pub fn delete(tx: &Transaction, id: i64) -> Result<usize> {
    Ok(tx.execute("DELETE FROM `profile_bookmark` WHERE `id` = ?", [id])?)
}
