use sqlite::Connection;
use std::{path::Path, rc::Rc, sync::RwLock};

pub struct Database {
    connection: Rc<RwLock<Connection>>,
}

impl Database {
    // Constructors

    pub fn new(path: &Path) -> Self {
        Self {
            connection: match Connection::open(path) {
                Ok(connection) => Rc::new(RwLock::new(connection)),
                Err(e) => panic!("Failed to connect profile database: {e}"),
            },
        }
    }

    // Getters

    pub fn connection(&self) -> &Rc<RwLock<Connection>> {
        &self.connection
    }
}
