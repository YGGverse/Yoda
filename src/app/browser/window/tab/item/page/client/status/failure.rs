// Global dependencies
use std::fmt::{Display, Formatter, Result};

/// Local `Failure` status for `Client`
pub enum Failure {
    /// Redirection count limit reached by protocol driver or global settings
    RedirectLimit { count: usize, is_global: bool },
}

impl Failure {
    // Constructors

    /// Create new `Self::RedirectLimit`
    pub fn redirect_limit(count: usize, is_global: bool) -> Self {
        Self::RedirectLimit { count, is_global }
    }
}

impl Display for Failure {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::RedirectLimit { count, is_global } => {
                if *is_global {
                    write!(f, "Redirection limit ({count}) reached by global settings")
                } else {
                    write!(
                        f,
                        "Redirection limit ({count}) reached by protocol restrictions"
                    )
                }
            }
        }
    }
}
