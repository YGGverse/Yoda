mod error;
pub use error::Error;

use crate::profile::Profile;
use gtk::{gio::TlsCertificate, prelude::TlsCertificateExt};
use std::rc::Rc;

/// Certificate details holder for export to file action
pub struct Certificate {
    pub data: String,
    pub name: String,
}

impl Certificate {
    // Constructors

    /// Create new `Self`
    pub fn new(profile: Rc<Profile>, profile_identity_gemini_id: i64) -> Result<Self, Error> {
        match profile
            .identity
            .gemini
            .database
            .record(profile_identity_gemini_id)
        {
            Ok(record) => match record {
                Some(identity) => match TlsCertificate::from_pem(&identity.pem) {
                    Ok(certificate) => Ok(Self {
                        data: identity.pem,
                        name: certificate.subject_name().unwrap().replace("CN=", ""),
                    }),
                    Err(e) => Err(Error::TlsCertificate(e)),
                },
                None => Err(Error::NotFound(profile_identity_gemini_id)),
            },
            Err(e) => Err(Error::Database(e)),
        }
    }
}
