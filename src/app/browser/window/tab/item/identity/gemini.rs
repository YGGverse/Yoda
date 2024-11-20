mod widget;
use widget::Widget;

use crate::app::browser::window::Action;
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
    pub fn new(profile: Rc<Profile>, action: Rc<Action>, auth_uri: Uri) -> Self {
        // Init widget
        let widget = Rc::new(Widget::new());

        // Init shared components
        let auth_url = auth_uri.to_string();

        // Add new identity option
        widget
            .form
            .list
            .append(None, "Create new..", "Auto-generated certificate");

        // Collect additional options from database
        match profile.identity.gemini.database.records() {
            Ok(identities) => {
                for identity in identities {
                    // Get certificate details
                    let certificate = match TlsCertificate::from_pem(&identity.pem) {
                        Ok(certificate) => certificate,
                        Err(reason) => todo!("{reason}"),
                    };

                    // Append record option
                    widget.form.list.append(
                        Some(identity.id),
                        &certificate.subject_name().unwrap().replace("CN=", ""), // trim prefix
                        &format!(
                            "valid: {} | auth: {}",
                            certificate
                                .not_valid_after()
                                .unwrap()
                                .format("%Y-%m-%d")
                                .unwrap(),
                            profile
                                .identity
                                .gemini
                                .auth
                                .database
                                .records(Some(auth_url.as_str()))
                                .unwrap()
                                .iter()
                                .filter(|this| this.profile_identity_gemini_id == identity.id)
                                .count(),
                        ),
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

                // Activate identity for given `auth_uri`
                match profile
                    .identity
                    .gemini
                    .auth
                    .activate(profile_identity_gemini_id, auth_url.as_str())
                {
                    Ok(_) => action.reload().activate(),
                    Err(reason) => todo!("{:?}", reason),
                }
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
