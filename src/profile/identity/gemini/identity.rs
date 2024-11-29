mod error;
use error::Error;

use gtk::gio::TlsCertificate;

pub struct Identity {
    pub pem: String,
    // pub scope: String,
}

impl Identity {
    pub fn to_tls_certificate(&self) -> Result<TlsCertificate, Error> {
        match TlsCertificate::from_pem(&self.pem) {
            Ok(certificate) => Ok(certificate),
            Err(reason) => Err(Error::TlsCertificate(reason)),
        }
    }
}
