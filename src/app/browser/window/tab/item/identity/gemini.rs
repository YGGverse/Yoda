mod widget;
use widget::Widget;

use crate::profile::Profile;
use gtk::{
    gio::{prelude::TlsCertificateExt, TlsCertificate},
    glib::Uri,
    prelude::IsA,
};
use std::rc::Rc;

pub struct Gemini {
    // profile: Rc<Profile>,
    widget: Rc<Widget>,
}

impl Gemini {
    // Construct

    /// Create new `Self` for given Profile
    pub fn new(profile: Rc<Profile>, auth_uri: Uri) -> Self {
        // Init widget
        let widget = Rc::new(Widget::new());

        // Add new identity option
        widget.form.list.append(None, "Create new..", true);

        // Collect additional options from database
        match profile.identity.gemini.database.records() {
            Ok(identities) => {
                for identity in identities {
                    // Get certificate details
                    let certificate = match TlsCertificate::from_pem(&identity.pem) {
                        Ok(certificate) => certificate,
                        Err(reason) => todo!("{reason}"),
                    };

                    // Get name from subject
                    let name = certificate.subject_name().unwrap();

                    // Get expiration time
                    let expires = certificate
                        .not_valid_after()
                        .unwrap()
                        .format_iso8601()
                        .unwrap();

                    // Append record option
                    widget.form.list.append(
                        Some(identity.id),
                        &format!("{name} ({expires})"),
                        true,
                    );
                }
            }
            Err(_) => todo!(),
        }

        // Init events
        widget.on_apply({
            let widget = widget.clone();
            move |response| {
                // Get record ID depending of user selection
                let profile_identity_gemini_id = match response {
                    // Use selected identity
                    Some(id) => id,
                    // Create new identity, get last insert ID
                    None => profile
                        .identity
                        .gemini
                        .create(None, widget.form.name.value().as_deref())
                        .unwrap(), // @TODO
                };

                // Apply identity for given `auth_uri`
                profile
                    .identity
                    .gemini
                    .auth
                    .apply(profile_identity_gemini_id, auth_uri.to_string().as_str())
                    .unwrap(); //@TODO handle errors

                // Reload page
                // @TODO
            }
        });

        // Return activated `Self`
        Self {
            // profile,
            widget,
        }
    }

    // Actions

    /// Show dialog for parent [Widget](https://docs.gtk.org/gtk4/class.Widget.html)
    pub fn present(&self, parent: Option<&impl IsA<gtk::Widget>>) {
        self.widget.present(parent);
    }
}
