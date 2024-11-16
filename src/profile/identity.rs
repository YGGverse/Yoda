mod gemini;
use gemini::Gemini;

use sqlite::{Connection, Transaction};
use std::{rc::Rc, sync::RwLock};

/// Authorization wrapper for different protocols
pub struct Identity {
    gemini: Rc<Gemini>,
}

impl Identity {
    // Constructors

    /// Create new `Self`
    pub fn new(connection: Rc<RwLock<Connection>>, profile_id: Rc<i64>) -> Self {
        Self {
            gemini: Rc::new(Gemini::new(connection, profile_id)),
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
                        if gemini_record.id == auth_record.gemini_id {
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
    // nothing yet..

    // Delegate migration to childs
    gemini::migrate(tx)?;

    // Success
    Ok(())
}
