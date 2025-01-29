use super::{
    list::{item::Value, List},
    WidgetAction,
};
use crate::Profile;
use adw::{
    prelude::{AdwDialogExt, AlertDialogExt, AlertDialogExtManual},
    AlertDialog, ResponseAppearance,
};
use gtk::{
    glib::Uri,
    prelude::{ButtonExt, WidgetExt},
    Button,
};
use std::rc::Rc;

// Defaults

const LABEL: &str = "Disconnect";
const TOOLTIP_TEXT: &str = "Stop use selected identity everywhere";
const MARGIN: i32 = 8;

const HEADING: &str = "Disconnect";
const BODY: &str = "Stop use selected identity for all scopes?";
const RESPONSE_CANCEL: (&str, &str) = ("cancel", "Cancel");
const RESPONSE_CONFIRM: (&str, &str) = ("confirm", "Confirm");

pub struct Exit {
    pub button: Button,
}

impl Exit {
    // Constructors

    /// Create new `Self`
    pub fn build(
        widget_action: &Rc<WidgetAction>,
        profile: &Rc<Profile>,
        list: &Rc<List>,
        request: &Uri,
    ) -> Self {
        // Init main widget
        let button = Button::builder()
            .label(LABEL)
            .margin_top(MARGIN)
            .tooltip_text(TOOLTIP_TEXT)
            .visible(false)
            .build();

        // Init events
        button.connect_clicked({
            let button = button.clone();
            let list = list.clone();
            let profile = profile.clone();
            let request = request.clone();
            let widget_action = widget_action.clone();
            move |_| {
                // Get selected identity from holder
                match list.selected().value_enum() {
                    Value::ProfileIdentityId(profile_identity_id) => {
                        // Init sub-widget
                        let alert_dialog = AlertDialog::builder()
                            .heading(HEADING)
                            .body(BODY)
                            .close_response(RESPONSE_CANCEL.0)
                            .default_response(RESPONSE_CANCEL.0)
                            .build();

                        // Set response variants
                        alert_dialog.add_responses(&[RESPONSE_CANCEL, RESPONSE_CONFIRM]);

                        // Decorate default response preset
                        alert_dialog.set_response_appearance(
                            RESPONSE_CONFIRM.0,
                            ResponseAppearance::Suggested,
                        );

                        /* contrast issue with Ubuntu orange accents
                        alert_dialog.set_response_appearance(
                            RESPONSE_CANCEL.0,
                            ResponseAppearance::Destructive,
                        ); */

                        // Connect confirmation event
                        alert_dialog.connect_response(Some(RESPONSE_CONFIRM.0), {
                            let button = button.clone();
                            let list = list.clone();
                            let profile = profile.clone();
                            let request = request.clone();
                            let widget_action = widget_action.clone();
                            move |_, _| {
                                match profile.identity.auth.remove_ref(profile_identity_id) {
                                    Ok(_) => {
                                        match list.selected().update(&profile, &request.to_string())
                                        {
                                            Ok(_) => {
                                                button.set_css_classes(&["success"]);
                                                button
                                                    .set_label("Identity successfully disconnected")
                                            }
                                            Err(e) => {
                                                button.set_css_classes(&["error"]);
                                                button.set_label(&e.to_string())
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        button.set_css_classes(&["error"]);
                                        button.set_label(&e.to_string())
                                    }
                                }
                                widget_action.update.activate();
                            }
                        });

                        // Show dialog
                        alert_dialog.present(Some(&button))
                    }
                    _ => todo!(), // unexpected
                }
            }
        });

        // Return activated `Self`
        Self { button }
    }

    // Actions

    pub fn update(&self, is_visible: bool, is_sensitive: bool) {
        self.button.set_visible(is_visible);
        self.button.set_sensitive(is_sensitive);
    }
}
