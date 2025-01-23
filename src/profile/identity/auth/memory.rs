pub mod auth;
pub mod error;

pub use auth::Auth;
pub use error::Error;

use std::{cell::RefCell, collections::HashMap};

/// Reduce disk usage by cache Auth index in memory
pub struct Memory {
    index: RefCell<HashMap<String, i64>>,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            index: RefCell::new(HashMap::new()),
        }
    }

    // Actions

    /// Add new record with `scope` as key and `profile_identity_id` as value
    /// * validate record with same key does not exist yet
    pub fn add(&self, scope: String, profile_identity_id: i64) -> Result<(), Error> {
        // Borrow shared index access
        let mut index = self.index.borrow_mut();

        // Prevent existing key overwrite
        if index.contains_key(&scope) {
            return Err(Error::Overwrite(scope));
        }

        // Slot should be free, let check it twice
        match index.insert(scope, profile_identity_id) {
            Some(_) => Err(Error::Unexpected),
            None => Ok(()),
        }
    }

    /// Cleanup index
    pub fn clear(&self) -> Result<(), Error> {
        let mut index = self.index.borrow_mut();
        index.clear();
        if index.is_empty() {
            Ok(())
        } else {
            Err(Error::Clear)
        }
    }

    /// Get identity match `request`
    /// * [Client certificates specification](https://geminiprotocol.net/docs/protocol-specification.gmi#client-certificates)
    /// * contain unspecified length priority implementation @TODO
    pub fn match_scope(&self, request: &str) -> Option<Auth> {
        let mut result = Vec::new();

        // Get all records starts with `scope`
        let query = filter_scope(request);

        for (scope, &profile_identity_id) in self.index.borrow().iter() {
            if query.starts_with(scope) {
                result.push(Auth {
                    profile_identity_id,
                    scope: scope.clone(),
                })
            }
        }

        // Sort by length desc @TODO
        result.sort_by(|a, b| b.scope.len().cmp(&a.scope.len()));

        // Get first copy
        result.first().cloned()
    }
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
