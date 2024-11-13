use sqlite::{Error, Transaction};

pub struct Table {
    pub id: i64,
    //pub profile_id: i64,
}

pub fn init(tx: &Transaction) -> Result<usize, Error> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_identity`
        (
            `id`          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_id`  INTEGER NOT NULL,
            `time`        INTEGER NOT NULL,
            `name`        VARCHAR(255),
            `certificate` TEXT NOT NULL,
            `key`         TEXT NOT NULL
        )",
        [],
    )
}
