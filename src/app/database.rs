use sqlite::Connection;
use std::sync::Arc;

pub struct Database {
    connection: Arc<sqlite::Connection>,
}

impl Database {
    // Construct new application DB
    pub fn init(connection: Arc<Connection>) -> Database {
        // Init app table
        if let Err(error) = connection.execute(
            r"
                CREATE TABLE IF NOT EXISTS `app`
                (
                    `id`   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                    `time` INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
                )
            ",
        ) {
            panic!("{error}");
        }

        // Return struct
        Self { connection }
    }

    // Restore previous browser session from DB
    pub fn restore(&self) {
        // @TODO migration test
    }

    // Save browser session to DB
    pub fn save(&self) {
        // @TODO migration test
    }
}
