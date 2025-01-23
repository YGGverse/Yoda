//! Controller for children `database` and `memory` components

mod database;
mod error;
mod memory;

use database::Database;
pub use error::Error;
use memory::Memory;

use sqlite::{Connection, Transaction};
use std::{rc::Rc, sync::RwLock};

/// API for `profile_identity_id` + `scope` auth pairs operations
pub struct Auth {
    pub database: Rc<Database>,
    pub memory: Rc<Memory>,
}

impl Auth {
    // Constructors

    /// Create new `Self`
    pub fn build(connection: &Rc<RwLock<Connection>>) -> Result<Self, Error> {
        // Init `Self`
        let this = Self {
            database: Rc::new(Database::build(connection)),
            memory: Rc::new(Memory::new()),
        };

        // Build initial index
        Self::index(&this)?;

        // Done
        Ok(this)
    }

    // Actions

    /// Apply `profile_identity_id` certificate as the auth for `scope`
    /// * deactivate active auth by remove previous records from `Self` database
    /// * reindex `Self` memory index on success
    /// * return last insert `profile_identity_auth_id` on success
    pub fn apply(&self, profile_identity_id: i64, request: &str) -> Result<i64, Error> {
        // Cleanup records match `scope` (unauthorize)
        self.remove(request)?;

        // Create new record (auth)
        let profile_identity_auth_id = match self
            .database
            .add(profile_identity_id, &filter_scope(request))
        {
            Ok(id) => id,
            Err(e) => return Err(Error::Database(e)),
        };

        // Reindex
        self.index()?;

        // Done
        Ok(profile_identity_auth_id)
    }

    /// Remove all records match request (unauthorize)
    pub fn remove(&self, request: &str) -> Result<(), Error> {
        match self.database.records_scope(Some(&filter_scope(request))) {
            Ok(records) => {
                for record in records {
                    if let Err(e) = self.database.delete(record.id) {
                        return Err(Error::Database(e));
                    }
                }
            }
            Err(e) => return Err(Error::Database(e)),
        }
        self.index()?;
        Ok(())
    }

    /// Remove all records match `profile_identity_id` foreign reference key
    pub fn remove_ref(&self, profile_identity_id: i64) -> Result<(), Error> {
        match self.database.records_ref(profile_identity_id) {
            Ok(records) => {
                for record in records {
                    if let Err(e) = self.database.delete(record.id) {
                        return Err(Error::Database(e));
                    }
                }
            }
            Err(e) => return Err(Error::Database(e)),
        }
        self.index()?;
        Ok(())
    }

    /// Create new `Memory` index from `Database` for `Self`
    pub fn index(&self) -> Result<(), Error> {
        // Clear previous records
        if let Err(e) = self.memory.clear() {
            return Err(Error::Memory(e));
        }

        // Build new index
        match self.database.records_scope(None) {
            Ok(records) => {
                for record in records {
                    if let Err(e) = self.memory.add(record.scope, record.profile_identity_id) {
                        return Err(Error::Memory(e));
                    }
                }
            }
            Err(e) => return Err(Error::Database(e)),
        }

        Ok(())
    }

    // Getters

    /// Check request string matches condition
    pub fn is_matches(&self, request: &str, profile_identity_id: i64) -> bool {
        self.memory
            .match_scope(&filter_scope(request))
            .is_some_and(|auth| auth.profile_identity_id == profile_identity_id)
    }

    /// Get memory item string match request
    pub fn get(&self, request: &str) -> Option<memory::Auth> {
        self.memory.match_scope(&filter_scope(request))
    }
}

// Tools

pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    // nothing yet..

    Ok(())
}

/// Get valid identity scope for given URL
/// * helper function for different protocol drivers implementation
fn filter_scope(url: &str) -> String {
    use gtk::glib::{Regex, RegexCompileFlags, RegexMatchFlags};

    match Regex::split_simple(
        r"^\w+://(.*)",
        url,
        RegexCompileFlags::DEFAULT,
        RegexMatchFlags::DEFAULT,
    )
    .get(1)
    {
        Some(postfix) => postfix.to_string(),
        None => url.to_string(),
    }
    .trim()
    .trim_end_matches("/")
    .to_lowercase()
}
