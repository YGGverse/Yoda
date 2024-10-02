use sqlite::Connection;
use std::sync::Arc;

enum Table {
    Id,
    Time,
}

pub struct Database {
    connection: Arc<sqlite::Connection>,
}

impl Database {
    // Construct new application DB
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
            panic!("{error}");
        }

        // Return struct
        Self { connection }
    }

    pub fn add(&self) -> i64 {
        if let Err(error) = self.connection.execute("INSERT INTO `app`", []) {
            panic!("{error}");
        }

        self.connection.last_insert_rowid()
    }
}
