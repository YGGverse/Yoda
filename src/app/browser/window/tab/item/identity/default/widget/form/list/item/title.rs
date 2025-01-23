use gtk::{gio::TlsCertificate, glib::gformat, prelude::TlsCertificateExt};

pub fn new_for_profile_identity_id(certificate: &TlsCertificate) -> String {
    certificate
        .subject_name()
        .unwrap_or(gformat!("Unknown"))
        .replace("CN=", "")
}
