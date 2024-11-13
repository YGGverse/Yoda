use sqlite::{Error, Transaction};

pub struct Table {
    pub id: i64,
    // pub app_browser_window_tab_item_page_id: i64, not in use
    pub title: Option<String>, // can be stored as NULL
}

pub fn init(tx: &Transaction) -> Result<usize, Error> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS `app_browser_window_tab_item_page_meta`
        (
            `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `app_browser_window_tab_item_page_id` INTEGER NOT NULL,
            `title` VARCHAR(1024)
        )",
        [],
    )
}

pub fn insert(
    tx: &Transaction,
    app_browser_window_tab_item_page_id: &i64,
    title: Option<&str>,
) -> Result<usize, Error> {
    tx.execute(
        "INSERT INTO `app_browser_window_tab_item_page_meta` (
            `app_browser_window_tab_item_page_id`,
            `title`
        ) VALUES (?, ?)",
        (app_browser_window_tab_item_page_id, title),
    )
}

pub fn select(
    tx: &Transaction,
    app_browser_window_tab_item_page_id: &i64,
) -> Result<Vec<Table>, Error> {
    let mut stmt = tx.prepare(
        "SELECT `id`,
                `app_browser_window_tab_item_page_id`,
                `title`
                FROM `app_browser_window_tab_item_page_meta`
                WHERE `app_browser_window_tab_item_page_id` = ?",
    )?;

    let result = stmt.query_map([app_browser_window_tab_item_page_id], |row| {
        Ok(Table {
            id: row.get(0)?,
            // app_browser_window_tab_item_page_id: row.get(1)?, not in use
            title: row.get(2)?,
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
    tx.execute(
        "DELETE FROM `app_browser_window_tab_item_page_meta` WHERE `id` = ?",
        [id],
    )
}

/* not in use
pub fn last_insert_id(tx: &Transaction) -> i64 {
    tx.last_insert_rowid()
} */
