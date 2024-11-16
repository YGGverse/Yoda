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

    /// Get `pem` record match `request`
    ///
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#client-certificates
    pub fn gemini(&self, request: &str) -> Option<String> {
        if let Ok(auth_records) = self.gemini.auth.database.records(Some(request)) {
            for auth_record in auth_records {
                if let Ok(gemini_records) = self.gemini.database.records() {
                    for gemini_record in gemini_records {
                        if gemini_record.id == auth_record.profile_identity_gemini_id {
                            return Some(gemini_record.pem);
                        }
                    }
                }
            }
        }
        None
    } // @TODO apply protocol rules to selection
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
