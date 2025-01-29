use gtk::{gio::TlsCertificate, prelude::TlsCertificateExt};

const DATE_FORMAT: &str = "%Y.%m.%d";

pub fn new_for_profile_identity_id(certificate: &TlsCertificate, scope: &[String]) -> String {
    format!(
        "{} - {} | scope: {}",
        certificate
            .not_valid_before()
            .unwrap() // @TODO
            .format(DATE_FORMAT)
            .unwrap(),
        certificate
            .not_valid_after()
            .unwrap() // @TODO
            .format(DATE_FORMAT)
            .unwrap(),
        scope.len(),
    )
}
