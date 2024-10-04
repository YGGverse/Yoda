use sqlite::{Connection, Error};
use std::sync::Arc;

pub struct Table {
    pub id: i64,
    pub app_id: i64,
    // pub time: i64,
    pub width: i32,
    pub height: i32,
    pub is_fullscreen: bool,
}

pub struct Database {
    connection: Arc<Connection>,
}

impl Database {
    pub fn init(connection: Arc<Connection>) -> Result<Database, Error> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS `app_browser`
            (
                `id`   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `time` INTEGER NOT NULL DEFAULT (UNIXEPOCH('NOW')),
                `app_id` INTEGER NOT NULL,
                `width` INTEGER NOT NULL,
                `height` INTEGER NOT NULL,
                `is_fullscreen` INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(Self { connection })
    }

    pub fn add(
        &self,
        app_id: i64,
        width: i32,
        height: i32,
        is_fullscreen: bool,
    ) -> Result<usize, Error> {
        self.connection.execute(
            "INSERT INTO `app_browser` (
                `app_id`,
                `width`,
                `height`,
                `is_fullscreen`
            ) VALUES (?, ?, ?, ?)",
            [
                app_id,
                width as i64,
                height as i64,
                match is_fullscreen {
                    true => 1,
                    false => 0,
                },
            ],
        )
    }

    pub fn records(&self, app_id: i64) -> Result<Vec<Table>, Error> {
        let mut statement = self.connection.prepare(
            "SELECT `id`,
                    `app_id`,
                    `width`,
                    `height`,
                    `is_fullscreen` FROM `app_browser` WHERE `app_id` = ?",
        )?;

        let result = statement.query_map([app_id], |row| {
            Ok(Table {
                id: row.get(0)?,
                app_id: row.get(1)?,
                width: row.get(2)?,
                height: row.get(3)?,
                is_fullscreen: row.get(4)?,
            })
        })?;

        let mut records = Vec::new();

        for record in result {
            let table = record?;
            records.push(table);
        }

        Ok(records)
    }

    pub fn delete(&self, id: i64) -> Result<usize, Error> {
        self.connection
            .execute("DELETE FROM `app_browser` WHERE `id` = ?", [id])
    }

    pub fn last_insert_id(&self) -> i64 {
        self.connection.last_insert_rowid()
    }
}
