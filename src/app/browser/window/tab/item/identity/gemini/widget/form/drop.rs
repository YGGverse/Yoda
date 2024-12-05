use super::Action;
use super::List;
use crate::profile::Profile;
use adw::{
    prelude::{AdwDialogExt, AlertDialogExt, AlertDialogExtManual},
    AlertDialog, ResponseAppearance,
};
use gtk::{
    prelude::{ButtonExt, WidgetExt},
    Button,
};
use std::{cell::RefCell, rc::Rc};

// Defaults

const LABEL: &str = "Delete";
const TOOLTIP_TEXT: &str = "Drop selected identity from profile";
const MARGIN: i32 = 8;

const HEADING: &str = "Delete";
const BODY: &str = "Delete selected identity from profile?";
const RESPONSE_CANCEL: (&str, &str) = ("cancel", "Cancel");
const RESPONSE_CONFIRM: (&str, &str) = ("confirm", "Confirm");

pub struct Drop {
    profile_identity_gemini_id: Rc<RefCell<Option<i64>>>,
    pub button: Button,
}

impl Drop {
    // Constructors

    /// Create new `Self`
    pub fn new(profile: Rc<Profile>, action: Rc<Action>, list: Rc<List>) -> Self {
        // Init selected option holder
        let profile_identity_gemini_id = Rc::new(RefCell::new(None::<i64>));

        // Init main widget
        let button = Button::builder()
            .label(LABEL)
            .margin_top(MARGIN)
            .tooltip_text(TOOLTIP_TEXT)
            .visible(false)
            .build();

        // Init events
        button.connect_clicked({
            let action = action.clone();
            let button = button.clone();
            let profile_identity_gemini_id = profile_identity_gemini_id.clone();
            move |_| {
                // Get selected identity from holder
                match profile_identity_gemini_id.borrow().as_ref() {
                    Some(profile_identity_gemini_id) => {
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
                            let action = action.clone();
                            let button = button.clone();
                            let list = list.clone();
                            let profile = profile.clone();
                            let profile_identity_gemini_id = *profile_identity_gemini_id;
                            move |_, _| {
                                match profile.identity.gemini.delete(profile_identity_gemini_id) {
                                    Ok(_) => {
                                        if list.remove(profile_identity_gemini_id).is_some() {
                                            button.set_css_classes(&["success"]);
                                            button.set_label("Identity successfully deleted")
                                        } else {
                                            button.set_css_classes(&["error"]);
                                            button.set_label("List item not found")
                                            // @TODO unexpected
                                        }
                                    }
                                    Err(e) => {
                                        button.set_css_classes(&["error"]);
                                        button.set_label(&e.to_string())
                                    }
                                }
                                action.update.activate()
                            }
                        });

                        // Show dialog
                        alert_dialog.present(Some(&button))
                    }
                    None => todo!(), // unexpected
                }
            }
        });

        // Return activated `Self`
        Self {
            profile_identity_gemini_id,
            button,
        }
    }

    // Actions

    /// Update `profile_identity_gemini_id` holder,
    /// toggle visibility depending on given value
    pub fn update(&self, profile_identity_gemini_id: Option<i64>) {
        self.button.set_visible(match profile_identity_gemini_id {
            Some(value) => {
                self.profile_identity_gemini_id.replace(Some(value));
                true
            }
            None => {
                self.profile_identity_gemini_id.replace(None);
                false
            }
        })
    }
}
