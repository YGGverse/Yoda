use sqlite::{Error, Transaction};

pub struct Table {
    pub id: i64,
}

pub fn init(tx: &Transaction) -> Result<usize, Error> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS `app`
        (
            `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL
        )",
        [],
    )
}

pub fn insert(tx: &Transaction) -> Result<usize, Error> {
    tx.execute("INSERT INTO `app` DEFAULT VALUES", [])
}

pub fn select(tx: &Transaction) -> Result<Vec<Table>, Error> {
    let mut stmt = tx.prepare("SELECT `id` FROM `app`")?;
    let result = stmt.query_map([], |row| Ok(Table { id: row.get(0)? }))?;

    let mut records = Vec::new();

    for record in result {
        let table = record?;
        records.push(table);
    }

    Ok(records)
}

pub fn delete(tx: &Transaction, id: &i64) -> Result<usize, Error> {
    tx.execute("DELETE FROM `app` WHERE `id` = ?", [id])
}

pub fn last_insert_id(tx: &Transaction) -> i64 {
    tx.last_insert_rowid()
}
