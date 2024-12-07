use super::list::{item::Value, List};
use crate::app::browser::action::Action as BrowserAction;
use crate::profile::Profile;
use adw::{
    prelude::{AdwDialogExt, AlertDialogExt, AlertDialogExtManual},
    AlertDialog, ResponseAppearance,
};
use gtk::{
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
    pub fn new(browser_action: Rc<BrowserAction>, profile: Rc<Profile>, list: Rc<List>) -> Self {
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
            move |_| {
                // Get selected identity from holder
                match list.selected().value_enum() {
                    Value::ProfileIdentityGeminiId(profile_identity_gemini_id) => {
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

                        alert_dialog.set_response_appearance(
                            RESPONSE_CANCEL.0,
                            ResponseAppearance::Destructive,
                        );

                        // Connect confirmation event
                        alert_dialog.connect_response(Some(RESPONSE_CONFIRM.0), {
                            let browser_action = browser_action.clone();
                            let button = button.clone();
                            let list = list.clone();
                            let profile = profile.clone();
                            move |_, _| {
                                match profile
                                    .identity
                                    .gemini
                                    .auth
                                    .remove_ref(profile_identity_gemini_id)
                                {
                                    Ok(_) => match list.selected().update(&profile, "") {
                                        Ok(_) => {
                                            button.set_css_classes(&["success"]);
                                            button.set_label("Identity successfully disconnected")
                                        }
                                        Err(e) => {
                                            button.set_css_classes(&["error"]);
                                            button.set_label(&e.to_string())
                                        }
                                    },
                                    Err(e) => {
                                        button.set_css_classes(&["error"]);
                                        button.set_label(&e.to_string())
                                    }
                                }
                                browser_action.update.activate(None)
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

    pub fn set_visible(&self, is_visible: bool) {
        self.button.set_visible(is_visible)
    }
}
