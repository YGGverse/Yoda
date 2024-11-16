use sqlite::{Error, Transaction};

pub struct Table {
    pub id: i64,
    //pub profile_id: i64,
}

// Low-level DB API

pub fn init(tx: &Transaction) -> Result<usize, Error> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS `profile_identity`
        (
            `id`         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            `profile_id` INTEGER NOT NULL,
            `time`       INTEGER NOT NULL,
            `name`       VARCHAR(255),
            `pem`        TEXT NOT NULL
        )",
        [],
    )
}
