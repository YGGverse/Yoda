use sqlite::{Error, Transaction};

pub struct Table {
    pub id: i64,
    // pub app_browser_window_tab_id: i64, not in use
    pub is_selected: bool,
}

pub struct Database {
    // nothing yet..
}

impl Database {
    pub fn init(tx: &Transaction) -> Result<usize, Error> {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS `app_browser_window_tab_item`
            (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `app_browser_window_tab_id` INTEGER NOT NULL,
                `page_position` INTEGER NOT NULL,
                `is_selected` INTEGER NOT NULL
            )",
            [],
        )
    }

    pub fn add(
        tx: &Transaction,
        app_browser_window_tab_id: &i64,
        page_position: &i32,
        is_selected: &bool,
    ) -> Result<usize, Error> {
        tx.execute(
            "INSERT INTO `app_browser_window_tab_item` (
                `app_browser_window_tab_id`,
                `page_position`,
                `is_selected`
            ) VALUES (?, ?, ?)",
            [
                app_browser_window_tab_id,
                &(*page_position as i64),
                &(*is_selected as i64),
            ],
        )
    }

    pub fn records(tx: &Transaction, app_browser_window_tab_id: &i64) -> Result<Vec<Table>, Error> {
        let mut stmt = tx.prepare(
            "SELECT `id`,
                    `app_browser_window_tab_id`,
                    `is_selected`
                    FROM `app_browser_window_tab_item`
                    WHERE `app_browser_window_tab_id` = ?
                    ORDER BY `page_position` ASC", // just order by, no store in struct wanted
        )?;

        let result = stmt.query_map([app_browser_window_tab_id], |row| {
            Ok(Table {
                id: row.get(0)?,
                // app_browser_window_tab_id: row.get(1)?, not in use
                is_selected: row.get(2)?,
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
            "DELETE FROM `app_browser_window_tab_item` WHERE `id` = ?",
            [id],
        )
    }

    pub fn last_insert_id(tx: &Transaction) -> i64 {
        tx.last_insert_rowid()
    }
}
