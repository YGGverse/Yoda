use sqlite::{Connection, Error};
use std::sync::Arc;

pub struct Table {
    pub id: i64,
    pub app_id: i64,
}

pub struct Database {
    connection: Arc<Connection>,
}

impl Database {
    pub fn init(connection: Arc<Connection>) -> Result<Database, Error> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS `app_browser`
            (
                `id`     INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `app_id` INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(Self { connection })
    }

    pub fn add(&self, app_id: &i64) -> Result<usize, Error> {
        self.connection
            .execute("INSERT INTO `app_browser` (`app_id`) VALUES (?)", [app_id])
    }

    pub fn records(&self, app_id: &i64) -> Result<Vec<Table>, Error> {
        let mut statement = self
            .connection
            .prepare("SELECT `id`, `app_id` WHERE `app_id` = ?")?;

        let result = statement.query_map([app_id], |row| {
            Ok(Table {
                id: row.get(0)?,
                app_id: row.get(1)?,
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
            .execute("DELETE FROM `app_browser` WHERE `id` = ?", [id])
    }

    pub fn last_insert_id(&self) -> i64 {
        self.connection.last_insert_rowid()
    }
}
