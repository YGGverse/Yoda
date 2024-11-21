mod auth;
mod certificate;
mod database;
mod error;
mod memory;

use auth::Auth;
use database::Database;
pub use error::Error;

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
            Err(reason) => return Err(Error::AuthInit(reason)),
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

    /// Create new record
    /// * return new `profile_identity_gemini_id` on success
    pub fn create(&self, time: Option<(DateTime, DateTime)>, name: &str) -> Result<i64, Error> {
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
            Ok(pem) => match self.database.add(&pem) {
                Ok(profile_identity_gemini_id) => {
                    self.index()?;
                    Ok(profile_identity_gemini_id)
                }
                Err(reason) => Err(Error::DatabaseRecordCreate(reason)),
            },
            Err(reason) => Err(Error::Certificate(reason)),
        }
    }

    /// Create new `Memory` index from `Database` for `Self`
    pub fn index(&self) -> Result<(), Error> {
        // Cleanup previous records
        self.memory.clear();

        // Build new index
        match self.database.records() {
            Ok(records) => {
                for record in records {
                    if self.memory.add(record.id, record.pem).is_err() {
                        return Err(Error::MemoryIndex(record.id));
                    }
                }
            }
            Err(reason) => return Err(Error::DatabaseIndex(reason)),
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
