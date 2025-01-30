use sqlite::{Error, Transaction};

pub struct Table {
    pub id: i64,
    // pub app_browser_window_tab_item_id: i64, not in use,
    pub is_needs_attention: bool,
    pub title: Option<String>,
}

pub fn init(tx: &Transaction) -> Result<usize, Error> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS `app_browser_window_tab_item_page`
        (
            `id`                             INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `app_browser_window_tab_item_id` INTEGER NOT NULL,
            `is_needs_attention`             INTEGER NOT NULL,
            `title`                          TEXT,

            FOREIGN KEY (`app_browser_window_tab_item_id`) REFERENCES `app_browser_window_tab_item` (`id`)
        )",
        [],
    )
}

pub fn insert(
    tx: &Transaction,
    app_browser_window_tab_item_id: i64,
    is_needs_attention: bool,
    title: Option<&str>,
) -> Result<usize, Error> {
    tx.execute(
        "INSERT INTO `app_browser_window_tab_item_page` (
            `app_browser_window_tab_item_id`,
            `is_needs_attention`,
            `title`
        ) VALUES (?, ?, ?)",
        (app_browser_window_tab_item_id, is_needs_attention, title),
    )
}

pub fn select(tx: &Transaction, app_browser_window_tab_item_id: i64) -> Result<Vec<Table>, Error> {
    let mut stmt = tx.prepare(
        "SELECT `id`,
                `app_browser_window_tab_item_id`,
                `is_needs_attention`,
                `title`
                FROM `app_browser_window_tab_item_page`
                WHERE `app_browser_window_tab_item_id` = ?",
    )?;

    let result = stmt.query_map([app_browser_window_tab_item_id], |row| {
        Ok(Table {
            id: row.get(0)?,
            // app_browser_window_tab_item_id: row.get(1)?, not in use
            is_needs_attention: row.get(2)?,
            title: row.get(3)?,
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
    tx.execute(
        "DELETE FROM `app_browser_window_tab_item_page` WHERE `id` = ?",
        [id],
    )
}

pub fn last_insert_id(tx: &Transaction) -> i64 {
    tx.last_insert_rowid()
}
