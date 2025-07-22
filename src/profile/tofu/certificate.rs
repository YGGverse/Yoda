use anyhow::Result;

use gtk::{
    gio::TlsCertificate,
    glib::{DateTime, GString},
};
use sourceview::prelude::TlsCertificateExt;

#[derive(PartialEq, Eq, Hash)]
pub struct Certificate {
    id: Option<i64>,
    time: DateTime,
    tls_certificate: TlsCertificate,
}

impl Certificate {
    // Constructors

    pub fn from_db(id: Option<i64>, pem: &str, time: DateTime) -> Result<Self> {
        Ok(Self {
            id,
            time,
            tls_certificate: TlsCertificate::from_pem(pem)?,
        })
    }

    pub fn from_tls_certificate(tls_certificate: TlsCertificate) -> Result<Self> {
        Ok(Self {
            id: None,
            time: DateTime::now_local()?,
            tls_certificate,
        })
    }

    // Getters

    pub fn pem(&self) -> GString {
        self.tls_certificate.certificate_pem().unwrap()
    }

    pub fn id(&self) -> Option<i64> {
        self.id
    }

    pub fn time(&self) -> &DateTime {
        &self.time
    }

    pub fn tls_certificate(&self) -> &TlsCertificate {
        &self.tls_certificate
    }
}
