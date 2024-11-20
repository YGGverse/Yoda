mod widget;
use widget::{form::list::item::value::Value, Widget};

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
        widget.form.list.append(
            Value::GENERATE_NEW_AUTH,
            "Create new",
            "Generate long-term certificate",
        );

        // Add guest option
        widget.form.list.append(
            Value::USE_GUEST_SESSION,
            "Guest session",
            "No identity for this request",
        );

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
                        Value::PROFILE_IDENTITY_GEMINI_ID(identity.id),
                        &certificate.subject_name().unwrap().replace("CN=", ""), // trim prefix
                        &format!(
                            "valid until {} | auth: {}",
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
                                .records(None)
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
                // Get option match user choice
                let option = match response {
                    Value::PROFILE_IDENTITY_GEMINI_ID(value) => Some(value),
                    Value::USE_GUEST_SESSION => None,
                    Value::GENERATE_NEW_AUTH => Some(
                        profile
                            .identity
                            .gemini
                            .create(None, widget.form.name.value().as_deref())
                            .unwrap(), // @TODO handle result,
                    ),
                };

                // Apply
                match option {
                    // Activate identity for `auth_uri`
                    Some(profile_identity_gemini_id) => {
                        profile
                            .identity
                            .gemini
                            .auth
                            .add(profile_identity_gemini_id, auth_url.as_str())
                            .unwrap();
                    }
                    // Remove all identity auths for `auth_uri`
                    None => {
                        profile
                            .identity
                            .gemini
                            .auth
                            .remove(auth_url.as_str())
                            .unwrap();
                    }
                }

                // Update page
                action.reload().activate();
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
