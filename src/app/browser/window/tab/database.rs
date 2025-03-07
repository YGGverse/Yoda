use anyhow::Result;
use sqlite::Transaction;

pub struct Table {
    pub id: i64,
    // pub app_browser_window_id: i64, not in use
}

pub fn init(tx: &Transaction) -> Result<usize> {
    Ok(tx.execute(
        "CREATE TABLE IF NOT EXISTS `app_browser_window_tab`
        (
            `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `app_browser_window_id` INTEGER NOT NULL,

            FOREIGN KEY (`app_browser_window_id`) REFERENCES `app_browser_window`(`id`)
        )",
        [],
    )?)
}

pub fn insert(tx: &Transaction, app_browser_window_id: i64) -> Result<i64> {
    tx.execute(
        "INSERT INTO `app_browser_window_tab` (
            `app_browser_window_id`
        ) VALUES (?)",
        [app_browser_window_id],
    )?;
    Ok(tx.last_insert_rowid())
}

pub fn select(tx: &Transaction, app_browser_window_id: i64) -> Result<Vec<Table>> {
    let mut stmt = tx.prepare(
        "SELECT `id`,
                `app_browser_window_id` FROM  `app_browser_window_tab`
                                        WHERE `app_browser_window_id` = ?",
    )?;

    let result = stmt.query_map([app_browser_window_id], |row| {
        Ok(Table {
            id: row.get(0)?,
            // app_browser_window_id: row.get(1)?, not in use
        })
    })?;

    let mut records = Vec::new();

    for record in result {
        let table = record?;
        records.push(table);
    }

    Ok(records)
}

pub fn delete(tx: &Transaction, id: i64) -> Result<usize> {
    Ok(tx.execute("DELETE FROM `app_browser_window_tab` WHERE `id` = ?", [id])?)
}
