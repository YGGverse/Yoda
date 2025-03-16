use anyhow::{Result, bail};
use gtk::gio::TlsCertificate;

/// Gemini identity holder for cached record in application-wide struct format.
/// Implements also additional conversion methods.
pub struct Item {
    pub pem: String,
    // pub scope: String,
}

impl Item {
    /// Convert `Self` to [TlsCertificate](https://docs.gtk.org/gio/class.TlsCertificate.html)
    pub fn to_tls_certificate(&self) -> Result<TlsCertificate> {
        match TlsCertificate::from_pem(&self.pem) {
            Ok(certificate) => Ok(certificate),
            Err(e) => bail!("TLS certificate error: {e}"),
        }
    }
}
