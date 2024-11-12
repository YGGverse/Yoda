mod bookmark;
mod history;
mod identity;

use bookmark::Bookmark;
use history::History;
use identity::Identity;

use sqlite::{Connection, Error};
use std::{
    path::Path,
    rc::Rc,
    sync::{RwLock, RwLockWriteGuard},
};

pub struct Database {
    connection: Rc<RwLock<Connection>>,
}

impl Database {
    // Constructors

    /// Create new connected `Self`
    pub fn new(path: &Path) -> Self {
        // Init database connection
        let connection = match Connection::open(path) {
            Ok(connection) => Rc::new(RwLock::new(connection)),
            Err(reason) => panic!("{reason}"),
        };

        // Init profile components
        match connection.try_write() {
            Ok(writable) => {
                if let Err(reason) = init(writable) {
                    panic!("{reason}")
                }
            }
            Err(reason) => panic!("{reason}"),
        };

        // Result
        Self { connection }
    }

    // Getters

    pub fn connection(&self) -> &Rc<RwLock<Connection>> {
        &self.connection
    }
}

// Tools

fn init(mut connection: RwLockWriteGuard<'_, Connection>) -> Result<(), Error> {
    // Create transaction
    let transaction = connection.transaction()?;

    // Init profile components
    Bookmark::init(&transaction)?;
    History::init(&transaction)?;
    Identity::init(&transaction)?;

    // Apply changes
    transaction.commit()?;

    Ok(())
}
