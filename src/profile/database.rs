use sqlite::{Error, Transaction};

use gtk::glib::DateTime;

pub struct Table {
    pub id: i64,
    pub active: bool,
    pub time: DateTime,
    pub name: Option<String>,
}

pub fn init(tx: &Transaction) -> Result<usize, Error> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile`
        (
            `id`     INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `active` INTEGER NOT NULL,
            `time`   INTEGER NOT NULL,
            `name`   VARCHAR(255)
        )",
        [],
    )
}

pub fn add(
    tx: &Transaction,
    active: bool,
    time: &DateTime,
    name: Option<&str>,
) -> Result<usize, Error> {
    tx.execute("INSERT INTO `profile`", (active, time.to_unix(), name))
}

pub fn records(tx: &Transaction) -> Result<Vec<Table>, Error> {
    let mut stmt = tx.prepare("SELECT `id`, `active`, `time`, `name` FROM `profile`")?;
    let result = stmt.query_map([], |row| {
        Ok(Table {
            id: row.get(0)?,
            active: row.get(1)?,
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

pub fn delete(tx: &Transaction, id: &i64) -> Result<usize, Error> {
    tx.execute("DELETE FROM `profile` WHERE `id` = ?", [id])
}

pub fn last_insert_id(tx: &Transaction) -> i64 {
    tx.last_insert_rowid()
}
