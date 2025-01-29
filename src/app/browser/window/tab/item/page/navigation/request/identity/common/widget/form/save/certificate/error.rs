use gtk::glib;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Database(sqlite::Error),
    NotFound(i64),
    TlsCertificate(glib::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Database(e) => {
                write!(f, "Database error: {e}")
            }
            Self::NotFound(profile_identity_id) => {
                write!(f, "Record for `{profile_identity_id}` not found")
            }
            Self::TlsCertificate(e) => {
                write!(f, "TLS certificate error: {e}")
            }
        }
    }
}
