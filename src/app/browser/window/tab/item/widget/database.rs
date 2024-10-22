use sqlite::{Error, Transaction};

pub struct Table {
    pub id: i64,
    // pub app_browser_window_tab_item_id: i64, not in use
    pub title: Option<String>, // can be stored as NULL
}

pub struct Database {
    // nothing yet..
}

impl Database {
    pub fn init(tx: &Transaction) -> Result<usize, Error> {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS `app_browser_window_tab_item_widget`
            (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `app_browser_window_tab_item_id` INTEGER NOT NULL,
                `title` VARCHAR(1024)
            )",
            [],
        )
    }

    pub fn add(
        tx: &Transaction,
        app_browser_window_tab_item_id: &i64,
        title: Option<&str>,
    ) -> Result<usize, Error> {
        tx.execute(
            "INSERT INTO `app_browser_window_tab_item_widget` (
                `app_browser_window_tab_item_id`,
                `title`
            ) VALUES (?, ?)",
            (app_browser_window_tab_item_id, title),
        )
    }

    pub fn records(
        tx: &Transaction,
        app_browser_window_tab_item_id: &i64,
    ) -> Result<Vec<Table>, Error> {
        let mut stmt = tx.prepare(
            "SELECT `id`,
                    `app_browser_window_tab_item_id`,
                    `title`
                    FROM `app_browser_window_tab_item_widget`
                    WHERE `app_browser_window_tab_item_id` = ?",
        )?;

        let result = stmt.query_map([app_browser_window_tab_item_id], |row| {
            Ok(Table {
                id: row.get(0)?,
                // app_browser_window_tab_item_id: row.get(1)?, not in use
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
            "DELETE FROM `app_browser_window_tab_item_widget` WHERE `id` = ?",
            [id],
        )
    }

    /* not in use
    pub fn last_insert_id(tx: &Transaction) -> i64 {
        tx.last_insert_rowid()
    } */
}
