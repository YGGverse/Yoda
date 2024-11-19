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

                    // Get expiration time
                    let expires = certificate
                        .not_valid_after()
                        .unwrap()
                        .format_iso8601()
                        .unwrap();

                    // Append record option
                    widget.form.list.append(
                        Some(identity.id),
                        &match identity.name {
                            Some(name) => format!("{name} ({expires})"),
                            None => format!("{expires}"),
                        },
                        true,
                    );
                }
            }
            Err(_) => todo!(),
        }

        // Init events
        widget.on_apply(move |response| match response {
            // Apply selected identity for `auth_uri`
            Some(profile_identity_gemini_id) => {
                profile
                    .identity
                    .gemini
                    .auth
                    .apply(profile_identity_gemini_id, auth_uri.to_string().as_str())
                    .unwrap(); //@TODO handle errors
            }
            // Create new certificate, then apply it to the new identity for `auth_uri`
            None => {}
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
