//! Controller for children `database` and `memory` components

mod database;
mod memory;

use anyhow::Result;
use database::Database;
use memory::Memory;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use sqlite::Transaction;

/// Auth pair operations
pub struct Auth {
    database: Database,
    memory: Memory,
}

impl Auth {
    // Constructors

    /// Create new `Self`
    pub fn build(database_pool: &Pool<SqliteConnectionManager>) -> Result<Self> {
        // Init `Self`
        let this = Self {
            database: Database::build(database_pool),
            memory: Memory::new(),
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
    pub fn apply(&self, profile_identity_id: i64, request: &str) -> Result<i64> {
        // Cleanup records match `scope` (unauthorize)
        self.remove(request)?;

        // Create new record (auth)
        let profile_identity_auth_id = self
            .database
            .add(profile_identity_id, &filter_scope(request))?;

        // Reindex
        self.index()?;

        // Done
        Ok(profile_identity_auth_id)
    }

    /// Remove all records match request (unauthorize)
    pub fn remove(&self, request: &str) -> Result<()> {
        for record in self.database.records_scope(Some(&filter_scope(request)))? {
            self.database.delete(record.id)?;
        }
        self.index()?;
        Ok(())
    }

    /// Remove all records match `profile_identity_id` foreign reference key
    pub fn remove_ref(&self, profile_identity_id: i64) -> Result<()> {
        for record in self.database.records_ref(profile_identity_id)? {
            self.database.delete(record.id)?;
        }
        self.index()?;
        Ok(())
    }

    /// Create new `Memory` index from `Database` for `Self`
    pub fn index(&self) -> Result<()> {
        // Clear previous records
        self.memory.clear()?;
        // Build new index
        for record in self.database.records_scope(None)? {
            self.memory.add(record.scope, record.profile_identity_id)?;
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

    /// Check request string matches condition
    pub fn total(&self, profile_identity_id: i64) -> usize {
        self.memory.total(profile_identity_id)
    }

    /// Collect certificate scope vector from `Profile` database for `profile_identity_id`
    pub fn scope(&self, profile_identity_id: i64) -> Vec<String> {
        let mut scope = Vec::new();
        match self.database.records_scope(None) {
            Ok(result) => {
                for auth in result
                    .iter()
                    .filter(|this| this.profile_identity_id == profile_identity_id)
                {
                    scope.push(auth.scope.clone())
                }
            }
            Err(_) => todo!(),
        }
        scope
    }

    /// Get memory item string match request
    pub fn get(&self, request: &str) -> Option<memory::Auth> {
        self.memory.match_scope(&filter_scope(request))
    }
}

// Tools

pub fn migrate(tx: &Transaction) -> Result<()> {
    // Migrate self components
    database::init(tx)?;

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
