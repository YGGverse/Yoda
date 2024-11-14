mod database;
use database::Database;

use sqlite::{Connection, Transaction};
use std::{rc::Rc, sync::RwLock};

pub struct Bookmark {
    pub database: Rc<Database>,
}

impl Bookmark {
    // Constructors

    pub fn new(connection: Rc<RwLock<Connection>>, profile_id: i64) -> Self {
        Self {
            database: Rc::new(Database::new(connection, profile_id)),
        }
    }
}

// Tools

pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    // nothing yet..

    // Success
    Ok(())
}
