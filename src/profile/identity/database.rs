use anyhow::Result;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use sqlite::Transaction;

pub struct Table {
    pub id: i64,
    //pub profile_id: i64,
    pub pem: String,
}

/// Storage for Gemini auth certificates
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

    // Actions

    /// Create new record in database
    pub fn add(&self, pem: &str) -> Result<i64> {
        // Begin new transaction
        let mut connection = self.pool.get()?;
        let tx = connection.transaction()?;

        // Create new record
        insert(&tx, self.profile_id, pem)?;

        // Hold insert ID for result
        let id = last_insert_id(&tx);

        // Done
        tx.commit()?;
        Ok(id)
    }

    /// Delete record with given `id` from database
    pub fn delete(&self, id: i64) -> Result<()> {
        // Begin new transaction
        let mut connection = self.pool.get()?;
        let tx = connection.transaction()?;

        // Create new record
        delete(&tx, id)?;

        // Done
        tx.commit()?;
        Ok(())
    }

    /// Get single record match `id`
    pub fn record(&self, id: i64) -> Result<Option<Table>> {
        let records = select(&self.pool.get()?.unchecked_transaction()?, self.profile_id)?; // @TODO single record query
        for record in records {
            if record.id == id {
                return Ok(Some(record));
            }
        }
        Ok(None)
    }

    /// Get all records match current `profile_id`
    pub fn records(&self) -> Result<Vec<Table>> {
        select(&self.pool.get()?.unchecked_transaction()?, self.profile_id)
    }
}

// Low-level DB API

pub fn init(tx: &Transaction) -> Result<usize> {
    Ok(tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_identity`
        (
            `id`         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_id` INTEGER NOT NULL,
            `pem`        TEXT NOT NULL,

            FOREIGN KEY (`profile_id`) REFERENCES `profile`(`id`)
        )",
        [],
    )?)
}

pub fn insert(tx: &Transaction, profile_id: i64, pem: &str) -> Result<usize> {
    Ok(tx.execute(
        "INSERT INTO `profile_identity` (
            `profile_id`,
            `pem`
        ) VALUES (?, ?)",
        (profile_id, pem),
    )?)
}

pub fn delete(tx: &Transaction, id: i64) -> Result<usize> {
    Ok(tx.execute("DELETE FROM `profile_identity` WHERE `id` = ?", [id])?)
}

pub fn select(tx: &Transaction, profile_id: i64) -> Result<Vec<Table>> {
    let mut stmt = tx.prepare(
        "SELECT `id`,
                `profile_id`,
                `pem`

        FROM `profile_identity` WHERE `profile_id` = ?",
    )?;

    let result = stmt.query_map([profile_id], |row| {
        Ok(Table {
            id: row.get(0)?,
            //profile_id: row.get(1)?,
            pem: row.get(2)?,
        })
    })?;

    let mut records = Vec::new();

    for record in result {
        let table = record?;
        records.push(table);
    }

    Ok(records)
}

pub fn last_insert_id(tx: &Transaction) -> i64 {
    tx.last_insert_rowid()
}
