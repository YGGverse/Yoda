mod database;

use database::Database;

use sqlite::{Connection, Transaction};
use std::{rc::Rc, sync::RwLock};

/// API for `profile_identity_gemini_id` + `url` auth pairs operations
pub struct Auth {
    pub database: Rc<Database>,
}

impl Auth {
    // Constructors

    /// Create new `Self`
    pub fn new(connection: Rc<RwLock<Connection>>) -> Self {
        Self {
            database: Rc::new(Database::new(connection)),
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
