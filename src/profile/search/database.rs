use anyhow::Result;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use sqlite::Transaction;

#[derive(Clone)]
pub struct Row {
    pub id: i64,
    //pub profile_id: i64,
    pub is_default: bool,
    pub query: String,
}

pub struct Database {
    pool: Pool<SqliteConnectionManager>,
    profile_id: i64,
}

impl Database {
    // Constructors

    /// Create new `Self`
    pub fn init(pool: &Pool<SqliteConnectionManager>, profile_id: i64) -> Result<Self> {
        let mut connection = pool.get()?;
        let tx = connection.transaction()?;

        let records = select(&tx, profile_id)?;

        if records.is_empty() {
            add_defaults(&tx, profile_id)?;
            tx.commit()?;
        }

        Ok(Self {
            pool: pool.clone(),
            profile_id,
        })
    }

    // Getters

    /// Get records from database
    pub fn records(&self) -> Result<Vec<Row>> {
        select(&self.pool.get()?.unchecked_transaction()?, self.profile_id)
    }

    // Setters

    /// Create new record in database
    /// * return last insert ID on success
    pub fn add(&self, query: String, is_default: bool) -> Result<i64> {
        let mut connection = self.pool.get()?;
        let tx = connection.transaction()?;
        if is_default {
            reset(&tx, self.profile_id, !is_default)?;
        }
        let id = insert(&tx, self.profile_id, query, is_default)?;
        tx.commit()?;
        Ok(id)
    }

    /// Delete record from database
    pub fn delete(&self, id: i64) -> Result<()> {
        // Begin new transaction
        let mut connection = self.pool.get()?;
        let tx = connection.transaction()?;

        // Delete record by ID
        delete(&tx, id)?;

        let records = select(&tx, self.profile_id)?;

        // Restore defaults if DB becomes empty
        if records.is_empty() {
            add_defaults(&tx, self.profile_id)?;
        } else {
            // At least one provider should be selected as default
            let mut has_default = false;
            for record in &records {
                if record.is_default {
                    has_default = true;
                    break;
                }
            }
            // Select first
            if !has_default {
                set_default(&tx, self.profile_id, records[0].id, true)?;
            }
        }

        // Done
        tx.commit()?;
        Ok(())
    }

    /// Delete record from database
    pub fn set_default(&self, id: i64) -> Result<()> {
        // Begin new transaction
        let mut connection = self.pool.get()?;
        let tx = connection.transaction()?;

        // Make sure only one default provider in set
        reset(&tx, self.profile_id, false)?;

        // Delete record by ID
        set_default(&tx, self.profile_id, id, true)?;
        tx.commit()?;
        Ok(())
    }
}

// Low-level DB API

pub fn init(tx: &Transaction) -> Result<usize> {
    Ok(tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_search`
        (
            `id`         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_id` INTEGER NOT NULL,
            `is_default` INTEGER NOT NULL,
            `query`      TEXT NOT NULL,

            FOREIGN KEY (`profile_id`) REFERENCES `profile` (`id`)
        )",
        [],
    )?)
}

fn insert(tx: &Transaction, profile_id: i64, query: String, is_default: bool) -> Result<i64> {
    tx.execute(
        "INSERT INTO `profile_search` (
            `profile_id`,
            `is_default`,
            `query`
        ) VALUES (?, ?, ?)",
        (profile_id, is_default, query),
    )?;
    Ok(tx.last_insert_rowid())
}

fn select(tx: &Transaction, profile_id: i64) -> Result<Vec<Row>> {
    let mut stmt = tx.prepare(
        "SELECT `id`, `profile_id`, `is_default`, `query`
            FROM `profile_search`
            WHERE `profile_id` = ?",
    )?;

    let result = stmt.query_map([profile_id], |row| {
        Ok(Row {
            id: row.get(0)?,
            //profile_id: row.get(1)?,
            is_default: row.get(2)?,
            query: row.get(3)?,
        })
    })?;

    let mut records = Vec::new();

    for record in result {
        let table = record?;
        records.push(table);
    }

    Ok(records)
}

fn delete(tx: &Transaction, id: i64) -> Result<usize> {
    Ok(tx.execute("DELETE FROM `profile_search` WHERE `id` = ?", [id])?)
}

fn reset(tx: &Transaction, profile_id: i64, is_default: bool) -> Result<usize> {
    Ok(tx.execute(
        "UPDATE `profile_search` SET `is_default` = ? WHERE `profile_id` = ?",
        (is_default, profile_id),
    )?)
}

fn set_default(tx: &Transaction, profile_id: i64, id: i64, is_default: bool) -> Result<usize> {
    Ok(tx.execute(
        "UPDATE `profile_search` SET `is_default` = ? WHERE `profile_id` = ? AND `id` = ?",
        (is_default, profile_id, id),
    )?)
}

/// Init default search providers list for given profile
fn add_defaults(tx: &Transaction, profile_id: i64) -> Result<()> {
    for (provider, is_default) in &[
        ("gemini://tlgs.one/search/search", true),
        ("gemini://kennedy.gemi.dev/search", false),
        ("gemini://auragem.ddns.net/search/s", false),
    ] {
        insert(tx, profile_id, provider.to_string(), *is_default)?;
    }
    Ok(())
}
