mod widget;
use widget::{form::list::item::value::Value, Widget};

use crate::app::browser::window::Action;
use crate::profile::Profile;
use gtk::{glib::Uri, prelude::IsA};
use std::rc::Rc;

pub struct Gemini {
    // profile: Rc<Profile>,
    widget: Rc<Widget>,
}

impl Gemini {
    // Construct

    /// Create new `Self` for given `Profile`
    pub fn new(profile: Rc<Profile>, action: Rc<Action>, auth_uri: Uri) -> Self {
        // Init shared URL string from URI
        let auth_url = auth_uri.to_string();

        // Init widget
        let widget = Rc::new(Widget::new(profile.clone(), &auth_url));

        // Init events
        widget.on_cancel({
            let action = action.clone();
            move |is_reload_request| {
                if is_reload_request {
                    action.reload.activate();
                }
            }
        });

        widget.on_apply({
            let widget = widget.clone();
            move |response| {
                // Get option match user choice
                let option = match response {
                    Value::ProfileIdentityGeminiId(value) => Some(value),
                    Value::GuestSession => None,
                    Value::GeneratePem => Some(
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
                            .apply(profile_identity_gemini_id, &auth_url)
                        {
                            todo!("{}", reason.to_string())
                        };
                    }
                    // Remove all identity auths for `auth_uri`
                    None => {
                        if let Err(reason) = profile.identity.gemini.auth.remove_scope(&auth_url) {
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
