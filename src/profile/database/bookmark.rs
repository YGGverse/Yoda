use sqlite::{Error, Transaction};

pub struct Table {
    pub id: i64,
    // pub app_id: i64, not in use
}

pub struct Bookmark {
    // nothing yet..
}

impl Bookmark {
    pub fn init(tx: &Transaction) -> Result<usize, Error> {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS `bookmark`
            (
                `id`   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `time` INTEGER NOT NULL,
                `data` TEXT
            )",
            [],
        )
    }
}
