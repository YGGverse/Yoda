mod auth;
mod database;
mod error;
mod memory;

use auth::Auth;
use database::Database;
use error::Error;
use memory::Memory;

use sqlite::{Connection, Transaction};
use std::{rc::Rc, sync::RwLock};

/// Authorization wrapper for Gemini protocol
///
/// https://geminiprotocol.net/docs/protocol-specification.gmi#client-certificates
pub struct Gemini {
    pub auth: Rc<Auth>,
    pub database: Rc<Database>,
    pub memory: Rc<Memory>,
    profile_identity_id: Rc<i64>,
}

impl Gemini {
    // Constructors

    /// Create new `Self`
    pub fn new(
        connection: Rc<RwLock<Connection>>,
        profile_identity_id: Rc<i64>,
    ) -> Result<Self, Error> {
        // Init components
        let auth = match Auth::new(connection.clone()) {
            Ok(auth) => Rc::new(auth),
            Err(_) => return Err(Error::AuthInit), // @TODO
        };
        let database = Rc::new(Database::new(connection, profile_identity_id.clone()));
        let memory = Rc::new(Memory::new());

        // Init `Self`
        let this = Self {
            auth,
            database,
            memory,
            profile_identity_id,
        };

        // Build initial index
        Self::index(&this)?;

        Ok(this)
    }

    // Actions

    /// Create new `Memory` index from `Database` for `Self`
    pub fn index(&self) -> Result<(), Error> {
        // Cleanup previous records
        self.memory.clear();

        // Build new index
        match self.database.records() {
            Ok(records) => {
                for record in records {
                    if self.memory.add(record.id, record.pem).is_err() {
                        return Err(Error::MemoryIndex); // @TODO
                    }
                }
            }
            Err(_) => return Err(Error::DatabaseIndex), // @TODO
        };
        Ok(()) // @TODO
    }
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
