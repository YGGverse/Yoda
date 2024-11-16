mod auth;
mod database;

use auth::Auth;
use database::Database;

use sqlite::{Connection, Transaction};
use std::{rc::Rc, sync::RwLock};

/// Authorization wrapper for Gemini protocol
///
/// https://geminiprotocol.net/docs/protocol-specification.gmi#client-certificates
pub struct Gemini {
    pub auth: Rc<Auth>,
    pub database: Rc<Database>,
}

impl Gemini {
    // Constructors

    /// Create new `Self`
    pub fn new(connection: Rc<RwLock<Connection>>, profile_identity_id: Rc<i64>) -> Self {
        Self {
            auth: Rc::new(Auth::new(connection.clone())),
            database: Rc::new(Database::new(connection, profile_identity_id)),
        }
    }

    // @TODO create new identity API
}

// Tools

pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    auth::migrate(tx)?;

    // Success
    Ok(())
}
