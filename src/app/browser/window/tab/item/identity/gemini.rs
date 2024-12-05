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

const DATE_FORMAT: &str = "%Y.%m.%d";

pub struct Gemini {
    // profile: Rc<Profile>,
    widget: Rc<Widget>,
}

impl Gemini {
    // Construct

    /// Create new `Self` for given Profile
    pub fn new(profile: Rc<Profile>, action: Rc<Action>, auth_uri: Uri) -> Self {
        // Init widget
        let widget = Rc::new(Widget::new(profile.clone()));

        // Init shared URL string from URI
        let url = auth_uri.to_string();

        // Add guest option
        widget.form.list.append(
            Value::UseGuestSession,
            "Guest session",
            "No identity for this request",
            None,
            false,
        );

        // Add new identity option
        widget.form.list.append(
            Value::GenerateNewAuth,
            "Create new",
            "Generate long-term certificate",
            None,
            false,
        );

        // Add import existing identity option
        widget.form.list.append(
            Value::ImportPem,
            "Import identity",
            "Use existing certificate",
            None,
            false,
        );

        // Collect identities as options from profile database
        // * memory cache synced also and could be faster @TODO
        match profile.identity.gemini.database.records() {
            Ok(identities) => {
                for identity in identities {
                    // Get certificate details
                    let certificate = match TlsCertificate::from_pem(&identity.pem) {
                        Ok(certificate) => certificate,
                        Err(reason) => todo!("{reason}"),
                    };

                    // Get auth details for tooltip
                    let mut auth_scope = Vec::new();

                    for auth in profile
                        .identity
                        .gemini
                        .auth
                        .database
                        .records_scope(None)
                        .unwrap()
                        .iter()
                        .filter(|this| this.profile_identity_gemini_id == identity.id)
                    {
                        auth_scope.push(format!("<small>{}</small>", auth.scope.clone()))
                    }

                    // Build tooltip
                    let mut tooltip = format!(
                        "<b>valid</b>\n<small>{}</small> - <small>{}</small>",
                        certificate
                            .not_valid_before()
                            .unwrap()
                            .format_iso8601()
                            .unwrap(),
                        certificate
                            .not_valid_after()
                            .unwrap()
                            .format_iso8601()
                            .unwrap()
                    );

                    if auth_scope.len() > 0 {
                        tooltip.push_str(&format!("\n\n<b>scope</b>\n{}", auth_scope.join("\n")));
                    }

                    // Append record option
                    widget.form.list.append(
                        Value::ProfileIdentityGeminiId(identity.id),
                        &certificate.subject_name().unwrap().replace("CN=", ""), // trim prefix
                        &format!(
                            "{} - {} | scope: {}",
                            certificate
                                .not_valid_before()
                                .unwrap()
                                .format(DATE_FORMAT)
                                .unwrap(),
                            certificate
                                .not_valid_after()
                                .unwrap()
                                .format(DATE_FORMAT)
                                .unwrap(),
                            auth_scope.len(),
                        ),
                        Some(&tooltip),
                        profile
                            .identity
                            .gemini
                            .auth
                            .memory
                            .match_scope(&url)
                            .is_some_and(|auth| auth.profile_identity_gemini_id == identity.id), // is selected
                    );
                }
            }
            Err(e) => todo!("{e}"),
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
                        match profile
                            .identity
                            .gemini
                            .make(None, &widget.form.name.value().unwrap())
                        {
                            Ok(profile_identity_gemini_id) => profile_identity_gemini_id,
                            Err(reason) => todo!("{}", reason.to_string()),
                        },
                    ),
                    Value::ImportPem => Some(
                        match profile
                            .identity
                            .gemini
                            .add(&widget.form.file.pem.take().unwrap())
                        {
                            Ok(profile_identity_gemini_id) => profile_identity_gemini_id,
                            Err(reason) => todo!("{}", reason.to_string()),
                        },
                    ),
                };

                // Apply auth
                match option {
                    // Activate identity for `auth_uri`
                    Some(profile_identity_gemini_id) => {
                        if let Err(reason) = profile
                            .identity
                            .gemini
                            .auth
                            .apply(profile_identity_gemini_id, &url)
                        {
                            todo!("{}", reason.to_string())
                        };
                    }
                    // Remove all identity auths for `auth_uri`
                    None => {
                        if let Err(reason) = profile.identity.gemini.auth.remove_scope(&url) {
                            todo!("{}", reason.to_string())
                        };
                    }
                }

                // Reload page to apply changes
                action.reload.activate();
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
