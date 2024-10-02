use std::sync::Arc;

pub struct Database {
    connection: Arc<sqlite::Connection>,
}

impl Database {
    // Construct new application DB
    pub fn init(connection: Arc<sqlite::Connection>) -> Database {
        // Init app table
        if let Err(e) = connection.execute(
            r"
                CREATE TABLE IF NOT EXISTS `app`
                (
                    `id`   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                    `time` INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
                )
            ",
        ) {
            panic!("{e}");
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
