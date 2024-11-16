mod database;
mod gemini;

use database::Database;
use gemini::Gemini;

use gtk::glib::DateTime;
use sqlite::{Connection, Transaction};
use std::{rc::Rc, sync::RwLock};

/// Authorization wrapper for different protocols
pub struct Identity {
    // database: Rc<Database>,
    gemini: Rc<Gemini>,
}

impl Identity {
    // Constructors

    /// Create new `Self`
    pub fn new(connection: Rc<RwLock<Connection>>, profile_id: Rc<i64>) -> Self {
        // Init identity database
        let database = Rc::new(Database::new(connection.clone()));

        // Get active identity set for profile or create new one
        let profile_identity_id = Rc::new(match database.active() {
            Some(identity) => identity.id,
            None => match database.add(profile_id, true, DateTime::now_local().unwrap(), None) {
                Ok(id) => id,
                Err(_) => todo!(),
            },
        });

        Self {
            // database,
            gemini: Rc::new(Gemini::new(connection, profile_identity_id)),
        }
    }

    /// Get `pem` record match `request` according to
    /// [Gemini protocol specification](https://geminiprotocol.net/docs/protocol-specification.gmi#client-certificates)
    /// * this function work with memory cache not database
    pub fn gemini(&self, request: &str) -> Option<String> {
        // @TODO apply protocol rules to certificate selection
        for profile_identity_gemini_id in self.gemini.auth.memory.starts_with(request) {
            if let Ok(pem) = self.gemini.memory.get(profile_identity_gemini_id) {
                return Some(pem);
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
    gemini::migrate(tx)?;

    // Success
    Ok(())
}
