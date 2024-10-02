use std::sync::Arc;

pub struct Database {
    connection: Arc<sqlite::Connection>,
    // Autostart migrate feature on app and db versions mismatch
    version: String,
}

impl Database {
    // Construct new application DB
    pub fn init(connection: Arc<sqlite::Connection>, version: &str) -> Database {
        // Create app table if not exist yet
        /*
        connection
            .execute(
                r"
                    CREATE TABLE IF NOT EXISTS `app`
                    (
                        `id`      INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                        `time`    INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        `version` VARCHAR NOT NULL
                    )
                ",
            )
            .unwrap(); // @TODO handle errors */

        // Return struct
        Self {
            connection,
            version: String::from(version),
        }
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
