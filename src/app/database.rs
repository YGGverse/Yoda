use sqlite::{Connection, Error};
use std::sync::Arc;

pub struct Table {
    pub id: i64,
    pub time: i64,
}

pub struct Database {
    connection: Arc<Connection>,
}

impl Database {
    pub fn init(connection: Arc<Connection>) -> Result<Database, Error> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS `app`
            (
                `id`   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `time` INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        Ok(Self { connection })
    }

    pub fn add(&self) -> Result<usize, Error> {
        self.connection
            .execute("INSERT INTO `app` DEFAULT VALUES", [])
    }

    pub fn records(&self) -> Result<Vec<Table>, Error> {
        let mut statement = self.connection.prepare("SELECT `id`, `time` FROM `app`")?;

        let result = statement.query_map([], |row| {
            Ok(Table {
                id: row.get(0)?,
                time: row.get(1)?,
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
            .execute("DELETE FROM `app` WHERE `id` = ?", [id])
    }

    pub fn last_insert_id(&self) -> i64 {
        self.connection.last_insert_rowid()
    }
}
