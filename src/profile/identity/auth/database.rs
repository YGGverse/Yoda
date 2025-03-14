use anyhow::Result;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use sqlite::Transaction;

pub struct Table {
    pub id: i64,
    pub profile_identity_id: i64,
    pub scope: String,
}

/// Storage for `profile_identity_id` + `scope` auth pairs
pub struct Database {
    pool: Pool<SqliteConnectionManager>,
}

impl Database {
    // Constructors

    /// Create new `Self`
    pub fn build(pool: &Pool<SqliteConnectionManager>) -> Self {
        Self { pool: pool.clone() }
    }

    // Actions

    /// Create new record in database
    pub fn add(&self, profile_identity_id: i64, scope: &str) -> Result<i64> {
        let mut connection = self.pool.get()?;
        let tx = connection.transaction()?;
        let id = insert(&tx, profile_identity_id, scope)?;
        tx.commit()?;
        Ok(id)
    }

    /// Delete record with given `id` from database
    pub fn delete(&self, id: i64) -> Result<()> {
        let mut connection = self.pool.get()?;
        let tx = connection.transaction()?;
        delete(&tx, id)?;
        tx.commit()?;
        Ok(())
    }

    // Getters

    /// Get records from database match current `profile_id` optionally filtered by `scope`
    pub fn records_scope(&self, scope: Option<&str>) -> Result<Vec<Table>> {
        select_scope(&self.pool.get()?.unchecked_transaction()?, scope)
    }

    /// Get records from database match current `profile_id` optionally filtered by `scope`
    pub fn records_ref(&self, profile_identity_id: i64) -> Result<Vec<Table>> {
        select_ref(
            &self.pool.get()?.unchecked_transaction()?,
            profile_identity_id,
        )
    }
}

// Low-level DB API

pub fn init(tx: &Transaction) -> Result<usize> {
    Ok(tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_identity_auth`
        (
            `id`                  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_identity_id` INTEGER NOT NULL,
            `scope`               VARCHAR(1024) NOT NULL,

            FOREIGN KEY (`profile_identity_id`) REFERENCES `profile_identity`(`id`),
            UNIQUE (`scope`)
        )",
        [],
    )?)
}

pub fn insert(tx: &Transaction, profile_identity_id: i64, scope: &str) -> Result<i64> {
    tx.execute(
        "INSERT INTO `profile_identity_auth` (
            `profile_identity_id`,
            `scope`
        ) VALUES (?, ?)",
        (profile_identity_id, scope),
    )?;
    Ok(tx.last_insert_rowid())
}

pub fn delete(tx: &Transaction, id: i64) -> Result<usize> {
    Ok(tx.execute("DELETE FROM `profile_identity_auth` WHERE `id` = ?", [id])?)
}

pub fn select_scope(tx: &Transaction, scope: Option<&str>) -> Result<Vec<Table>> {
    let mut stmt = tx.prepare(
        "SELECT `id`,
                `profile_identity_id`,
                `scope`

                FROM `profile_identity_auth`
                WHERE `scope` LIKE ?",
    )?;

    let result = stmt.query_map([scope.unwrap_or("%")], |row| {
        Ok(Table {
            id: row.get(0)?,
            profile_identity_id: row.get(1)?,
            scope: row.get(2)?,
        })
    })?;

    let mut records = Vec::new();

    for record in result {
        let table = record?;
        records.push(table);
    }

    Ok(records)
}

pub fn select_ref(tx: &Transaction, profile_identity_id: i64) -> Result<Vec<Table>> {
    let mut stmt = tx.prepare(
        "SELECT `id`,
                `profile_identity_id`,
                `scope`

                FROM `profile_identity_auth`
                WHERE `profile_identity_id` = ?",
    )?;

    let result = stmt.query_map([profile_identity_id], |row| {
        Ok(Table {
            id: row.get(0)?,
            profile_identity_id: row.get(1)?,
            scope: row.get(2)?,
        })
    })?;

    let mut records = Vec::new();

    for record in result {
        let table = record?;
        records.push(table);
    }

    Ok(records)
}
