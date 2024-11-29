use gtk::glib;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    TlsCertificate(glib::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::TlsCertificate(e) => write!(f, "TLS certificate error: {e}"),
        }
    }
}
