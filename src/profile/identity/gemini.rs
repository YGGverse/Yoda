mod auth;
mod certificate;
mod database;
mod error;
mod identity;
mod memory;

use auth::Auth;
use database::Database;
pub use error::Error;
use identity::Identity;

use memory::Memory;

use gtk::glib::DateTime;
use sqlite::{Connection, Transaction};
use std::{rc::Rc, sync::RwLock};

/// Authorization wrapper for Gemini protocol
///
/// https://geminiprotocol.net/docs/protocol-specification.gmi#client-certificates
pub struct Gemini {
    pub auth: Rc<Auth>,
    pub database: Rc<Database>,
    pub memory: Rc<Memory>,
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
            Err(e) => return Err(Error::Auth(e)),
        };
        let database = Rc::new(Database::new(connection, profile_identity_id.clone()));
        let memory = Rc::new(Memory::new());

        // Init `Self`
        let this = Self {
            auth,
            database,
            memory,
        };

        // Build initial index
        Self::index(&this)?;

        Ok(this)
    }

    // Actions

    /// Add new record to database, update memory index
    /// * return new `profile_identity_gemini_id` on success
    pub fn add(&self, pem: &str) -> Result<i64, Error> {
        match self.database.add(pem) {
            Ok(profile_identity_gemini_id) => {
                self.index()?;
                Ok(profile_identity_gemini_id)
            }
            Err(e) => Err(Error::Database(e)),
        }
    }

    /// Delete record from database including children dependencies, update memory index
    pub fn delete(&self, profile_identity_gemini_id: i64) -> Result<(), Error> {
        match self.auth.remove_ref(profile_identity_gemini_id) {
            Ok(_) => match self.database.delete(profile_identity_gemini_id) {
                Ok(_) => {
                    self.index()?;
                    Ok(())
                }
                Err(e) => Err(Error::Database(e)),
            },
            Err(e) => Err(Error::Auth(e)),
        }
    }

    /// Generate new certificate and insert record to DB, update memory index
    /// * return new `profile_identity_gemini_id` on success
    pub fn make(&self, time: Option<(DateTime, DateTime)>, name: &str) -> Result<i64, Error> {
        // Generate new certificate
        match certificate::generate(
            match time {
                Some(value) => value,
                None => (
                    DateTime::now_local().unwrap(),
                    DateTime::from_local(9999, 12, 31, 23, 59, 59.9).unwrap(), // max @TODO
                ),
            },
            name,
        ) {
            Ok(pem) => self.add(&pem),
            Err(e) => Err(Error::Certificate(e)),
        }
    }

    /// Create new `Memory` index from `Database` for `Self`
    pub fn index(&self) -> Result<(), Error> {
        // Clear previous records
        if let Err(e) = self.memory.clear() {
            return Err(Error::Memory(e));
        }

        // Build new index
        match self.database.records() {
            Ok(records) => {
                for record in records {
                    if let Err(e) = self.memory.add(record.id, record.pem) {
                        return Err(Error::Memory(e));
                    }
                }
            }
            Err(e) => return Err(Error::Database(e)),
        };

        Ok(())
    }

    /// Get `Identity` match `request`
    /// * [Client certificates specification](https://geminiprotocol.net/docs/protocol-specification.gmi#client-certificates)
    /// * this function work with memory cache (not database)
    pub fn match_scope(&self, request: &str) -> Option<Identity> {
        if let Some(auth) = self.auth.memory.match_scope(request) {
            match self.memory.get(auth.profile_identity_gemini_id) {
                Ok(pem) => {
                    return Some(Identity {
                        // scope: auth.scope,
                        pem,
                    });
                }
                Err(e) => todo!("{:?}", e.to_string()),
            }
        }
        None
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
