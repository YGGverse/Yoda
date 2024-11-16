mod database;
mod memory;

use database::Database;
use memory::Memory;

use sqlite::{Connection, Transaction};
use std::{rc::Rc, sync::RwLock};

/// API for `profile_identity_gemini_id` + `url` auth pairs operations
pub struct Auth {
    // pub database: Rc<Database>,
    pub memory: Rc<Memory>,
}

impl Auth {
    // Constructors

    /// Create new `Self`
    pub fn new(connection: Rc<RwLock<Connection>>) -> Self {
        // Init children components
        let database = Rc::new(Database::new(connection));
        let memory = Rc::new(Memory::new());

        // Build initial index
        match database.records(None) {
            Ok(records) => {
                for record in records {
                    if record.is_active {
                        if memory
                            .add(record.url, record.profile_identity_gemini_id)
                            .is_err()
                        {
                            todo!()
                        }
                    }
                }
            }
            Err(reason) => todo!("{reason}"),
        }

        // Return new `Self`
        Self {
            // database,
            memory,
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
