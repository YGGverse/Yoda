use gtk::glib::DateTime;
use sqlite::{Error, Transaction};

pub struct Table {
    pub id: i64,
    pub profile_id: i64,
    pub time: DateTime,
    pub request: String,
}

pub struct History {
    // nothing yet..
}

pub fn init(tx: &Transaction) -> Result<usize, Error> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_history`
        (
            `id`         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_id` INTEGER NOT NULL,
            `time`       INTEGER NOT NULL,
            `request`    TEXT NOT NULL
        )",
        [],
    )
}

pub fn add(
    tx: &Transaction,
    profile_id: &i64,
    time: &DateTime,
    request: &str,
) -> Result<usize, Error> {
    tx.execute(
        "INSERT INTO `history` (
            `profile_id`,
            `time`,
            `request`
        ) VALUES (?, ?, ?)",
        (profile_id, time.to_unix(), request),
    )
}

pub fn records(
    tx: &Transaction,
    profile_id: &i64,
    request: Option<&str>,
) -> Result<Vec<Table>, Error> {
    let mut stmt = tx.prepare(
        "SELECT `id`, `profile_id`, `time`, `request`
            FROM `profile_history`
            WHERE `profile_id` = ? AND `request` LIKE ?",
    )?;

    let filter = match request {
        Some(value) => value,
        None => "%",
    };

    let result = stmt.query_map((profile_id, filter), |row| {
        Ok(Table {
            id: row.get(0)?,
            profile_id: row.get(1)?,
            time: DateTime::from_unix_local(row.get(2)?).unwrap(),
            request: row.get(3)?,
        })
    })?;

    let mut records = Vec::new();

    for record in result {
        let table = record?;
        records.push(table);
    }

    Ok(records)
}

pub fn delete(tx: &Transaction, id: &i64) -> Result<usize, Error> {
    tx.execute("DELETE FROM `profile_history` WHERE `id` = ?", [id])
}
