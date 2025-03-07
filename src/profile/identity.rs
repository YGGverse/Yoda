mod auth;
mod certificate;
mod database;
mod item;
mod memory;

use anyhow::{bail, Result};
use auth::Auth;
use database::Database;
use gtk::glib::DateTime;
use item::Item;
use memory::Memory;
use sqlite::{Connection, Transaction};
use std::{rc::Rc, sync::RwLock};

/// Authorization wrapper for Gemini protocol
///
/// https://geminiprotocol.net/docs/protocol-specification.gmi#client-certificates
pub struct Identity {
    pub auth: Rc<Auth>,
    pub database: Rc<Database>,
    pub memory: Rc<Memory>,
}

impl Identity {
    // Constructors

    /// Create new `Self`
    pub fn build(
        connection: &Rc<RwLock<Connection>>,
        profile_identity_id: &Rc<i64>,
    ) -> Result<Self> {
        // Init components
        let auth = match Auth::build(connection) {
            Ok(auth) => Rc::new(auth),
            Err(e) => bail!("Could not create auth: {e}"),
        };
        let database = Rc::new(Database::build(connection, profile_identity_id));
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
    /// * return new `profile_identity_id` on success
    pub fn add(&self, pem: &str) -> Result<i64> {
        let profile_identity_id = self.database.add(pem)?;
        self.index()?;
        Ok(profile_identity_id)
    }

    /// Delete record from database including children dependencies, update memory index
    pub fn delete(&self, profile_identity_id: i64) -> Result<()> {
        self.auth.remove_ref(profile_identity_id)?;
        self.database.delete(profile_identity_id)?;
        self.index()?;
        Ok(())
    }

    /// Generate new certificate and insert record to DB, update memory index
    /// * return new `profile_identity_id` on success
    pub fn make(&self, time: Option<(DateTime, DateTime)>, name: &str) -> Result<i64> {
        // Generate new certificate
        match certificate::generate(
            match time {
                Some(value) => value,
                None => (
                    DateTime::now_local()?,
                    DateTime::from_local(9999, 12, 31, 23, 59, 59.9)?, // max @TODO
                ),
            },
            name,
        ) {
            Ok(pem) => self.add(&pem),
            Err(e) => bail!("Could not create certificate: {e}"),
        }
    }

    /// Create new `Memory` index from `Database` for `Self`
    pub fn index(&self) -> Result<()> {
        // Clear previous records
        self.memory.clear()?;
        for record in self.database.records()? {
            self.memory.add(record.id, record.pem)?;
        }
        Ok(())
    }

    /// Get `Identity` match `request`
    /// * [Client certificates specification](https://geminiprotocol.net/docs/protocol-specification.gmi#client-certificates)
    /// * this function work with memory cache (not database)
    pub fn get(&self, request: &str) -> Option<Item> {
        if let Some(auth) = self.auth.get(request) {
            match self.memory.get(auth.profile_identity_id) {
                Ok(pem) => {
                    return Some(Item {
                        // scope: auth.scope,
                        pem,
                    });
                }
                Err(e) => todo!("{e}"),
            }
        }
        None
    }
}

// Tools

pub fn migrate(tx: &Transaction) -> Result<()> {
    // Migrate self components
    database::init(tx)?;

    // Delegate migration to childs
    auth::migrate(tx)?;

    // Success
    Ok(())
}
