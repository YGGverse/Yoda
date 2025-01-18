use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Glib(gtk::glib::Error),
    Unsupported,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Glib(e) => write!(f, "{e}"),
            Self::Unsupported => write!(f, "Request not supported"),
        }
    }
}
