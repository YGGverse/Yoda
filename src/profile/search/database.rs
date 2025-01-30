use sqlite::{Connection, Error, Transaction};
use std::{rc::Rc, sync::RwLock};

#[derive(Clone)]
pub struct Row {
    pub id: i64,
    //pub profile_id: i64,
    pub is_default: bool,
    pub query: String,
}

pub struct Database {
    connection: Rc<RwLock<Connection>>,
    profile_id: Rc<i64>, // multi-profile relationship
}

impl Database {
    // Constructors

    /// Create new `Self`
    pub fn init(connection: &Rc<RwLock<Connection>>, profile_id: &Rc<i64>) -> Result<Self, Error> {
        let mut writable = connection.write().unwrap(); // @TODO handle
        let tx = writable.transaction()?;

        let records = select(&tx, **profile_id)?;

        if records.is_empty() {
            add_defaults(&tx, **profile_id)?;
            tx.commit()?;
        }

        Ok(Self {
            connection: connection.clone(),
            profile_id: profile_id.clone(),
        })
    }

    // Getters

    /// Get records from database
    pub fn records(&self) -> Result<Vec<Row>, Error> {
        let readable = self.connection.read().unwrap(); // @TODO handle
        let tx = readable.unchecked_transaction()?;
        select(&tx, *self.profile_id)
    }

    // Setters

    /// Create new record in database
    /// * return last insert ID on success
    pub fn add(&self, query: String, is_default: bool) -> Result<i64, Error> {
        // Begin new transaction
        let mut writable = self.connection.write().unwrap(); // @TODO handle
        let tx = writable.transaction()?;

        // Create new record
        if is_default {
            // make sure only one default provider in set
            reset(&tx, *self.profile_id, !is_default)?;
        }
        insert(&tx, *self.profile_id, query, is_default)?;

        // Hold insert ID for result
        let id = last_insert_id(&tx);

        // Done
        match tx.commit() {
            Ok(_) => Ok(id),
            Err(e) => Err(e),
        }
    }

    /// Delete record from database
    pub fn delete(&self, id: i64) -> Result<(), Error> {
        // Begin new transaction
        let mut writable = self.connection.write().unwrap(); // @TODO
        let tx = writable.transaction()?;

        // Delete record by ID
        delete(&tx, id)?;

        let records = select(&tx, *self.profile_id)?;

        // Restore defaults if DB becomes empty
        if records.is_empty() {
            add_defaults(&tx, *self.profile_id)?;
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
                set_default(&tx, *self.profile_id, records[0].id, true)?;
            }
        }

        // Done
        tx.commit()
    }

    /// Delete record from database
    pub fn set_default(&self, id: i64) -> Result<(), Error> {
        // Begin new transaction
        let mut writable = self.connection.write().unwrap(); // @TODO
        let tx = writable.transaction()?;

        // Make sure only one default provider in set
        reset(&tx, *self.profile_id, false)?;

        // Delete record by ID
        set_default(&tx, *self.profile_id, id, true)?;
        tx.commit()
    }
}

// Low-level DB API

pub fn init(tx: &Transaction) -> Result<usize, Error> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_search`
        (
            `id`         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_id` INTEGER NOT NULL,
            `is_default` INTEGER NOT NULL,
            `query`      TEXT NOT NULL,

            FOREIGN KEY (`profile_id`) REFERENCES `profile` (`id`)
        )",
        [],
    )
}

fn insert(
    tx: &Transaction,
    profile_id: i64,
    query: String,
    is_default: bool,
) -> Result<usize, Error> {
    tx.execute(
        "INSERT INTO `profile_search` (
            `profile_id`,
            `is_default`,
            `query`
        ) VALUES (?, ?, ?)",
        (profile_id, is_default, query),
    )
}

fn select(tx: &Transaction, profile_id: i64) -> Result<Vec<Row>, Error> {
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

fn delete(tx: &Transaction, id: i64) -> Result<usize, Error> {
    tx.execute("DELETE FROM `profile_search` WHERE `id` = ?", [id])
}

fn reset(tx: &Transaction, profile_id: i64, is_default: bool) -> Result<usize, Error> {
    tx.execute(
        "UPDATE `profile_search` SET `is_default` = ? WHERE `profile_id` = ?",
        (is_default, profile_id),
    )
}

fn set_default(
    tx: &Transaction,
    profile_id: i64,
    id: i64,
    is_default: bool,
) -> Result<usize, Error> {
    tx.execute(
        "UPDATE `profile_search` SET `is_default` = ? WHERE `profile_id` = ? AND `id` = ?",
        (is_default, profile_id, id),
    )
}

fn last_insert_id(tx: &Transaction) -> i64 {
    tx.last_insert_rowid()
}

/// Init default search providers list for given profile
fn add_defaults(tx: &Transaction, profile_id: i64) -> Result<(), Error> {
    for (provider, is_default) in &[
        ("gemini://kennedy.gemi.dev/search", true),
        ("gemini://tlgs.one/search/search", false),
    ] {
        insert(tx, profile_id, provider.to_string(), *is_default)?;
    }
    Ok(())
}
