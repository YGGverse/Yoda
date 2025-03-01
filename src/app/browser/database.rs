use sqlite::{Error, Transaction};

pub struct Table {
    pub id: i64,
    // pub app_id: i64, not in use
}

pub fn init(tx: &Transaction) -> Result<usize, Error> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS `app_browser`
        (
            `id`     INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `app_id` INTEGER NOT NULL,

            FOREIGN KEY (`app_id`) REFERENCES `app`(`id`)
        )",
        [],
    )
}

pub fn insert(tx: &Transaction, app_id: i64) -> Result<usize, Error> {
    tx.execute("INSERT INTO `app_browser` (`app_id`) VALUES (?)", [app_id])
}

pub fn select(tx: &Transaction, app_id: i64) -> Result<Vec<Table>, Error> {
    let mut stmt = tx.prepare("SELECT `id`, `app_id` FROM `app_browser` WHERE `app_id` = ?")?;

    let result = stmt.query_map([app_id], |row| {
        Ok(Table {
            id: row.get(0)?,
            // app_id: row.get(1)?, not in use
        })
    })?;

    let mut records = Vec::new();

    for record in result {
        let table = record?;
        records.push(table);
    }

    Ok(records)
}

pub fn delete(tx: &Transaction, id: i64) -> Result<usize, Error> {
    tx.execute("DELETE FROM `app_browser` WHERE `id` = ?", [id])
}

pub fn last_insert_id(tx: &Transaction) -> i64 {
    tx.last_insert_rowid()
}
