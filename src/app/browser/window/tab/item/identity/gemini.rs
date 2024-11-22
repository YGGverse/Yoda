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

        // Set first record selected by default
        let mut selected: u32 = 0;

        // Add guest option
        widget.form.list.append(
            Value::UseGuestSession,
            "Guest session",
            "No identity for this request",
        );

        // Add new identity option
        widget.form.list.append(
            Value::GenerateNewAuth,
            "Create new",
            "Generate long-term certificate",
        );

        // Add import existing identity option
        widget.form.list.append(
            Value::ImportPem,
            "Import identity",
            "Use existing certificate",
        );

        // Collect additional options from database
        let mut i = 2; // start from 3'th
        match profile.identity.gemini.database.records() {
            Ok(identities) => {
                for identity in identities {
                    i += 1;

                    // Get certificate details
                    let certificate = match TlsCertificate::from_pem(&identity.pem) {
                        Ok(certificate) => certificate,
                        Err(reason) => todo!("{reason}"),
                    };

                    // Append record option
                    widget.form.list.append(
                        Value::ProfileIdentityGeminiId(identity.id),
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

                    // Is selected?
                    if profile
                        .identity
                        .gemini
                        .auth
                        .database
                        .records(Some(auth_url.as_str()))
                        .unwrap()
                        .iter()
                        .filter(|this| this.profile_identity_gemini_id == identity.id)
                        .count()
                        > 0
                    {
                        selected = i;
                    }
                }

                // Select list item
                widget.form.list.gobject.set_selected(selected);
            }
            Err(_) => todo!(),
        }

        // Init events
        widget.on_apply({
            let widget = widget.clone();
            move |response| {
                // Get option match user choice
                let option = match response {
                    Value::ProfileIdentityGeminiId(value) => Some(value),
                    Value::UseGuestSession => None,
                    Value::GenerateNewAuth => Some(
                        profile
                            .identity
                            .gemini
                            .make(None, &widget.form.name.value().unwrap())
                            .unwrap(),
                    ),
                    Value::ImportPem => Some(
                        profile
                            .identity
                            .gemini
                            .add(&widget.form.file.pem.take().unwrap())
                            .unwrap(),
                    ),
                };

                // Apply auth
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

                // Reload page to apply changes
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
