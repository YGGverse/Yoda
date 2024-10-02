use sqlite::Connection;
use std::sync::Arc;

const DEBUG: bool = true; // @TODO

enum Table {
    Id,
    Time,
}

pub struct Database {
    connection: Arc<Connection>,
}

impl Database {
    pub fn init(connection: Arc<Connection>) -> Database {
        // Init app table
        if let Err(error) = connection.execute(
            "CREATE TABLE IF NOT EXISTS `app`
            (
                `id`   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `time` INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ) {
            panic!("{error}"); // @TODO
        }

        // Return struct
        Self { connection }
    }

    pub fn add(&self) -> Result<usize, String> {
        return match self.connection.execute("INSERT INTO `app`", []) {
            Ok(total) => {
                if DEBUG {
                    println!("Inserted {total} row to `app` table");
                }
                Ok(total)
            }
            Err(error) => Err(error.to_string()),
        };
    }

    pub fn clean(&self) -> Result<usize, String> {
        return match self.connection.execute("DELETE FROM `app`", []) {
            Ok(total) => {
                if DEBUG {
                    println!("Deleted {total} rows from `app` table");
                }
                Ok(total)
            }
            Err(error) => Err(error.to_string()),
        };
    }

    pub fn last_insert_id(&self) -> i64 {
        self.connection.last_insert_rowid()
    }
}
