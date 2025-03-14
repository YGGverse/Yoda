use anyhow::Result;
use gtk::glib::DateTime;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use sqlite::Transaction;

pub struct Table {
    pub id: i64,
    pub is_active: bool,
    pub time: DateTime,
    pub name: Option<String>,
}

pub struct Database {
    pub pool: Pool<SqliteConnectionManager>,
}

impl Database {
    // Constructors

    /// Create new `Self`
    pub fn build(pool: &Pool<SqliteConnectionManager>) -> Self {
        Self { pool: pool.clone() }
    }

    // Getters

    /// Get all records
    pub fn records(&self) -> Result<Vec<Table>> {
        select(&self.pool.get()?.unchecked_transaction()?)
    }

    /// Get active profile record if exist
    pub fn active(&self) -> Result<Option<Table>> {
        let records = self.records()?;
        Ok(records.into_iter().find(|record| record.is_active))
    }

    // Setters

    /// Create new record in `Self` database connected
    pub fn add(&self, is_active: bool, time: DateTime, name: Option<String>) -> Result<i64> {
        let mut connection = self.pool.get()?;
        let tx = connection.transaction()?;
        if is_active {
            for record in select(&tx)? {
                update(&tx, record.id, false, record.time, record.name)?;
            }
        }
        let id = insert(&tx, is_active, time, name)?;
        tx.commit()?;
        Ok(id)
    }
}

// Low-level DB API

pub fn init(tx: &Transaction) -> Result<usize> {
    Ok(tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile`
        (
            `id`        INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `is_active` INTEGER NOT NULL,
            `time`      INTEGER NOT NULL,
            `name`      VARCHAR(255)
        )",
        [],
    )?)
}

pub fn insert(
    tx: &Transaction,
    is_active: bool,
    time: DateTime,
    name: Option<String>,
) -> Result<i64> {
    tx.execute(
        "INSERT INTO `profile` (
            `is_active`,
            `time`,
            `name`
        ) VALUES (?, ?, ?)",
        (is_active, time.to_unix(), name),
    )?;
    Ok(tx.last_insert_rowid())
}

pub fn update(
    tx: &Transaction,
    id: i64,
    is_active: bool,
    time: DateTime,
    name: Option<String>,
) -> Result<usize> {
    Ok(tx.execute(
        "UPDATE `profile` SET `is_active` = ?, `time` = ?, `name` = ? WHERE `id` = ?",
        (is_active, time.to_unix(), name, id),
    )?)
}

pub fn select(tx: &Transaction) -> Result<Vec<Table>> {
    let mut stmt = tx.prepare("SELECT `id`, `is_active`, `time`, `name` FROM `profile`")?;
    let result = stmt.query_map([], |row| {
        Ok(Table {
            id: row.get(0)?,
            is_active: row.get(1)?,
            time: DateTime::from_unix_local(row.get(2)?).unwrap(),
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
