mod database;
mod error;
mod gemini;

use database::Database;
pub use error::Error;
use gemini::Gemini;

use sqlite::{Connection, Transaction};
use std::{rc::Rc, sync::RwLock};

/// Authorization wrapper for different protocols
pub struct Identity {
    // database: Rc<Database>,
    pub gemini: Rc<Gemini>,
}

impl Identity {
    // Constructors

    /// Create new `Self`
    pub fn build(connection: &Rc<RwLock<Connection>>, profile_id: &Rc<i64>) -> Result<Self, Error> {
        // Init identity database
        let database = Rc::new(Database::build(connection));

        // Get active identity set for profile or create new one
        let profile_identity_id = Rc::new(match database.active() {
            Ok(result) => match result {
                Some(identity) => identity.id,
                None => match database.add(profile_id, true) {
                    Ok(id) => id,
                    Err(e) => return Err(Error::Database(e)),
                },
            },
            Err(e) => return Err(Error::Database(e)),
        });

        // Init gemini component
        let gemini = Rc::new(match Gemini::build(connection, &profile_identity_id) {
            Ok(result) => result,
            Err(e) => return Err(Error::Gemini(e)),
        });

        // Done
        Ok(Self {
            // database,
            gemini,
        })
    }
}

// Tools

pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    gemini::migrate(tx)?;

    // Success
    Ok(())
}
