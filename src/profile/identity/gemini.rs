mod auth;
mod database;
mod memory;

use auth::Auth;
use database::Database;
use memory::Memory;

use sqlite::{Connection, Transaction};
use std::{rc::Rc, sync::RwLock};

/// Authorization wrapper for Gemini protocol
///
/// https://geminiprotocol.net/docs/protocol-specification.gmi#client-certificates
pub struct Gemini {
    pub auth: Rc<Auth>,
    // pub database: Rc<Database>,
    pub memory: Rc<Memory>,
}

impl Gemini {
    // Constructors

    /// Create new `Self`
    pub fn new(connection: Rc<RwLock<Connection>>, profile_identity_id: Rc<i64>) -> Self {
        // Init children components
        let auth = Rc::new(Auth::new(connection.clone()));
        let database = Rc::new(Database::new(connection, profile_identity_id));
        let memory = Rc::new(Memory::new());

        // Build initial index
        match database.records() {
            Ok(records) => {
                for record in records {
                    if memory.add(record.id, record.pem).is_err() {
                        todo!()
                    }
                }
            }
            Err(reason) => todo!("{reason}"),
        }

        Self {
            auth,
            // database,
            memory,
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
