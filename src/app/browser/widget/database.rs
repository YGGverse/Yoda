use sqlite::{Connection, Error};
use std::sync::Arc;

pub struct Table {
    pub id: i64,
    // pub app_browser_id: i64, not in use
    pub default_width: i32,
    pub default_height: i32,
    pub is_maximized: bool,
}

pub struct Database {
    connection: Arc<Connection>,
}

impl Database {
    pub fn init(connection: Arc<Connection>) -> Result<Database, Error> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS `app_browser_widget`
            (
                `id`             INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `app_browser_id` INTEGER NOT NULL,
                `default_width`  INTEGER NOT NULL,
                `default_height` INTEGER NOT NULL,
                `is_maximized`   INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(Self { connection })
    }

    pub fn add(
        &self,
        app_browser_id: &i64,
        default_width: &i32,
        default_height: &i32,
        is_maximized: &bool,
    ) -> Result<usize, Error> {
        self.connection.execute(
            "INSERT INTO `app_browser_widget` (
                `app_browser_id`,
                `default_width`,
                `default_height`,
                `is_maximized`
            ) VALUES (?, ?, ?, ?)",
            [
                app_browser_id,
                &(*default_width as i64),
                &(*default_height as i64),
                match is_maximized {
                    true => &1,
                    false => &0,
                },
            ],
        )
    }

    pub fn records(&self, app_browser_id: &i64) -> Result<Vec<Table>, Error> {
        let mut statement = self.connection.prepare(
            "SELECT `id`,
                    `app_browser_id`,
                    `default_width`,
                    `default_height`,
                    `is_maximized` FROM `app_browser_widget` WHERE `app_browser_id` = ?",
        )?;

        let result = statement.query_map([app_browser_id], |row| {
            Ok(Table {
                id: row.get(0)?,
                // app_browser_id: row.get(1)?, not in use
                default_width: row.get(2)?,
                default_height: row.get(3)?,
                is_maximized: row.get(4)?,
            })
        })?;

        let mut records = Vec::new();

        for record in result {
            let table = record?;
            records.push(table);
        }

        Ok(records)
    }

    pub fn delete(&self, id: &i64) -> Result<usize, Error> {
        self.connection
            .execute("DELETE FROM `app_browser_widget` WHERE `id` = ?", [id])
    }

    /* not in use
    pub fn last_insert_id(&self) -> i64 {
        self.connection.last_insert_rowid()
    } */
}
