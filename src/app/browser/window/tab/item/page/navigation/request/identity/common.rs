mod action;
mod form;

use crate::Profile;
use action::Action as WidgetAction;
use adw::AlertDialog;
use gtk::{glib::Uri, prelude::EditableExt};
use std::{rc::Rc, sync::Arc};

// Select options

pub trait Common {
    fn common(
        profile: &Arc<Profile>,
        request: &Uri,
        callback: &Rc<impl Fn(bool) + 'static>,
    ) -> Self;
}

impl Common for AlertDialog {
    // Constructors

    /// Create new `Self`
    fn common(
        profile: &Arc<Profile>,
        request: &Uri,
        callback: &Rc<impl Fn(bool) + 'static>,
    ) -> Self {
        use adw::{
            ResponseAppearance,
            prelude::{AlertDialogExt, AlertDialogExtManual},
        };
        use form::{Form, list::item::value::Value};

        // Response variants
        const RESPONSE_APPLY: (&str, &str) = ("apply", "Apply");
        const RESPONSE_CANCEL: (&str, &str) = ("cancel", "Cancel");
        // const RESPONSE_MANAGE: (&str, &str) = ("manage", "Manage");

        // Init actions
        let action = Rc::new(WidgetAction::new());

        // Init child container
        let form = Rc::new(Form::build(&action, profile, request));

        // Init main widget
        let alert_dialog = AlertDialog::builder()
            .heading("Identity")
            .body("Select identity certificate")
            .close_response(RESPONSE_CANCEL.0)
            .default_response(RESPONSE_APPLY.0)
            .extra_child(&form.g_box)
            .build();

        alert_dialog.add_responses(&[
            RESPONSE_CANCEL,
            // RESPONSE_MANAGE,
            RESPONSE_APPLY,
        ]);

        alert_dialog.connect_response(Some(RESPONSE_APPLY.0), {
            let callback = callback.clone();
            let form = form.clone();
            let profile = profile.clone();
            let request = request.clone();
            move |this, response| {
                // Prevent double-click action
                this.set_response_enabled(response, false);

                // Get option match user choice
                let option = match form.list.selected().value_enum() {
                    Value::ProfileIdentityId(value) => Some(value),
                    Value::GuestSession => None,
                    Value::GeneratePem => Some(
                        profile.identity.make(None, &form.name.text()).unwrap(), // @TODO handle
                    ),
                    Value::ImportPem => Some(
                        profile
                            .identity
                            .add(&form.file.pem.take().unwrap())
                            .unwrap(), // @TODO handle
                    ),
                };

                // Apply auth
                match option {
                    // Activate identity for `scope`
                    Some(profile_identity_id) => {
                        if profile
                            .identity
                            .auth
                            .apply(profile_identity_id, &request.to_string())
                            .is_err()
                        {
                            panic!() // unexpected @TODO
                        }
                    }
                    // Remove all identity auths for `scope`
                    None => {
                        if profile.identity.auth.remove(&request.to_string()).is_err() {
                            panic!() // unexpected @TODO
                        }
                    }
                }

                // Run callback function
                callback(true)
            }
        });

        // Deactivate not implemented feature @TODO
        // alert_dialog.set_response_enabled(RESPONSE_MANAGE.0, false);

        // Decorate default response preset
        alert_dialog.set_response_appearance(RESPONSE_APPLY.0, ResponseAppearance::Suggested);
        /* contrast issue with Ubuntu orange accents
        alert_dialog.set_response_appearance(RESPONSE_CANCEL.0, ResponseAppearance::Destructive); */

        // Init events
        action.update.connect_activate({
            let form = form.clone();
            let callback = callback.clone();
            let alert_dialog = alert_dialog.clone();
            move || {
                // Update child components
                form.update();

                // Deactivate apply button if the form values could not be processed
                alert_dialog.set_response_enabled(RESPONSE_APPLY.0, form.is_applicable());

                callback(false);
            }
        });

        // Make initial update
        action.update.activate();

        // Return new activated `Self`
        alert_dialog
    }
}
