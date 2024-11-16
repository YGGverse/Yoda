mod database;

use database::Database;

use sqlite::{Connection, Transaction};
use std::{rc::Rc, sync::RwLock};

/// API for `gemini_id` + `request` auth pairs operations
pub struct Auth {
    pub database: Rc<Database>,
}

impl Auth {
    // Constructors

    /// Create new `Self`
    pub fn new(connection: Rc<RwLock<Connection>>, profile_id: Rc<i64>) -> Self {
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
