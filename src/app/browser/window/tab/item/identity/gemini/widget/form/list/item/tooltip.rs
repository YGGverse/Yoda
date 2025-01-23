use gtk::{gio::TlsCertificate, prelude::TlsCertificateExt};

pub fn new_for_profile_identity_id(certificate: &TlsCertificate, scope: &[String]) -> String {
    let mut tooltip = "<b>Certificate</b>\n".to_string();

    if let Some(subject_name) = certificate.subject_name() {
        tooltip.push_str(&format!("\n<small><b>subject</b>\n{subject_name}</small>"));
    }

    if let Some(issuer_name) = certificate.issuer_name() {
        tooltip.push_str(&format!("\n<small><b>issuer</b>\n{issuer_name}</small>"));
    }

    if let Some(not_valid_before) = certificate.not_valid_before() {
        if let Ok(timestamp) = not_valid_before.format_iso8601() {
            tooltip.push_str(&format!("\n<small><b>valid after</b>\n{timestamp}</small>"));
        }
    }

    if let Some(not_valid_after) = certificate.not_valid_after() {
        if let Ok(timestamp) = not_valid_after.format_iso8601() {
            tooltip.push_str(&format!(
                "\n<small><b>valid before</b>\n{timestamp}</small>"
            ));
        }
    }

    if !scope.is_empty() {
        tooltip.push_str("\n\n<b>Scope</b>\n");

        for path in scope {
            tooltip.push_str(&format!("\n<small>{}</small>", path));
        }
    }

    tooltip
}
