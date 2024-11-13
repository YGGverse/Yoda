use sqlite::{Error, Transaction};

pub struct Table {
    pub id: i64,
}

pub fn init(tx: &Transaction) -> Result<usize, Error> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile`
        (
            `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL
        )",
        [],
    )
}

pub fn add(tx: &Transaction) -> Result<usize, Error> {
    tx.execute("INSERT INTO `profile` DEFAULT VALUES", [])
}

pub fn records(tx: &Transaction) -> Result<Vec<Table>, Error> {
    let mut stmt = tx.prepare("SELECT `profile_id` FROM `profile`")?;
    let result = stmt.query_map([], |row| Ok(Table { id: row.get(0)? }))?;

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
