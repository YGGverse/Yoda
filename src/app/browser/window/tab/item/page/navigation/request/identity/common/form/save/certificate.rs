use anyhow::{Result, bail};

use crate::profile::Profile;
use gtk::{gio::TlsCertificate, prelude::TlsCertificateExt};
use std::sync::Arc;

/// Certificate details holder for export to file action
pub struct Certificate {
    pub data: String,
    pub name: String,
}

impl Certificate {
    // Constructors

    /// Create new `Self`
    pub fn build(profile: &Arc<Profile>, profile_identity_id: i64) -> Result<Self> {
        let record = profile.identity.database.record(profile_identity_id)?;
        match record {
            Some(identity) => Ok(Self {
                name: TlsCertificate::from_pem(&identity.pem)?
                    .subject_name()
                    .unwrap_or_default()
                    .replace("CN=", ""),
                data: identity.pem,
            }),
            None => bail!("Identity not found!"),
        }
    }
}
