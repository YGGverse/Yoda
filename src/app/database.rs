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
        let mut records: Vec<Table> = Vec::new();

        let mut statement = self.connection.prepare("SELECT `id`, `time` FROM `app`")?;
        let _ = statement.query_map([], |row| {
            records.push(Table {
                id: row.get(0)?,
                time: row.get(1)?,
            });
            Ok(())
        });

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
