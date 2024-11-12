use gtk::glib::DateTime;
use sqlite::{Error, Transaction};

pub struct Table {
    pub id: i64,
    pub time: DateTime,
    pub request: String,
}

pub struct History {
    // nothing yet..
}

impl History {
    pub fn init(tx: &Transaction) -> Result<usize, Error> {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS `history`
            (
                `id`      INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `time`    INTEGER NOT NULL,
                `request` TEXT NOT NULL
            )",
            [],
        )
    }

    pub fn add(tx: &Transaction, time: &DateTime, request: &str) -> Result<usize, Error> {
        tx.execute(
            "INSERT INTO `history` (
                `time`,
                `request`
            ) VALUES (?, ?)",
            (time.to_unix(), request),
        )
    }

    pub fn records(tx: &Transaction, request: Option<&str>) -> Result<Vec<Table>, Error> {
        let mut stmt =
            tx.prepare("SELECT `id`, `time`, `request` FROM `history` WHERE `request` LIKE ?")?;

        let filter = match request {
            Some(value) => value,
            None => "%",
        };

        let result = stmt.query_map([filter], |row| {
            Ok(Table {
                id: row.get(0)?,
                time: DateTime::from_unix_local(row.get(1)?).unwrap(),
                request: row.get(2)?,
            })
        })?;

        let mut records = Vec::new();

        for record in result {
            let table = record?;
            records.push(table);
        }

        Ok(records)
    }

    pub fn delete(tx: &Transaction, id: &i64) -> Result<usize, Error> {
        tx.execute("DELETE FROM `history` WHERE `id` = ?", [id])
    }
}
