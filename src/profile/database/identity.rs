use sqlite::{Error, Transaction};

pub struct Table {
    pub id: i64,
    // pub app_id: i64, not in use
}

pub struct Identity {
    // nothing yet..
}

impl Identity {
    pub fn init(tx: &Transaction) -> Result<usize, Error> {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS `identity`
            (
                `id`   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `time` INTEGER NOT NULL,
                `name` VARCHAR(255),
                `crt`  TEXT NOT NULL,
                `key`  TEXT NOT NULL
            )",
            [],
        )
    }
}
