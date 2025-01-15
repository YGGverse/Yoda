// Global dependencies
use std::fmt::{Display, Formatter, Result};

/// Local `Failure` status for `Client`
pub enum Failure {
    /// Redirection count limit reached by protocol driver or global settings
    RedirectLimit { count: usize },
}

impl Failure {
    // Constructors

    /// Create new `Self::RedirectLimit`
    pub fn redirect_limit(count: usize) -> Self {
        Self::RedirectLimit { count }
    }
}

impl Display for Failure {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::RedirectLimit { count } => {
                write!(
                    f,
                    "Redirection limit ({count}) reached by protocol driver or global settings"
                )
            }
        }
    }
}
