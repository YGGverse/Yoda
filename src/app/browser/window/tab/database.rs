use sqlite::{Error, Transaction};

pub struct Table {
    pub id: i64,
    // pub app_browser_window_id: i64, not in use
    pub is_current: bool,
}

pub struct Database {
    // nothing yet..
}

impl Database {
    pub fn init(tx: &Transaction) -> Result<Database, Error> {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS `app_browser_window_tab`
            (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `app_browser_window_id` INTEGER NOT NULL,
                `is_current` INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(Self {})
    }

    pub fn add(
        &self,
        tx: &Transaction,
        app_browser_window_id: &i64,
        is_current: &bool,
    ) -> Result<usize, Error> {
        tx.execute(
            "INSERT INTO `app_browser_window_tab` (
                `app_browser_window_id`,
                `is_current`
            ) VALUES (?,?)",
            [app_browser_window_id, &(*is_current as i64)],
        )
    }

    pub fn records(
        &self,
        tx: &Transaction,
        app_browser_window_id: &i64,
    ) -> Result<Vec<Table>, Error> {
        let mut stmt = tx.prepare(
            "SELECT `id`,
                    `app_browser_window_id`,
                    `is_current` FROM `app_browser_window_tab`
                                WHERE `app_browser_window_id` = ?",
        )?;

        let result = stmt.query_map([app_browser_window_id], |row| {
            Ok(Table {
                id: row.get(0)?,
                // app_browser_window_id: row.get(1)?, not in use
                is_current: row.get(2)?,
            })
        })?;

        let mut records = Vec::new();

        for record in result {
            let table = record?;
            records.push(table);
        }

        Ok(records)
    }

    pub fn delete(&self, tx: &Transaction, id: &i64) -> Result<usize, Error> {
        tx.execute("DELETE FROM `app_browser_window_tab` WHERE `id` = ?", [id])
    }

    /* not in use
    pub fn last_insert_id(&self, tx: &Transaction) -> i64 {
        tx.last_insert_rowid()
    } */
}
