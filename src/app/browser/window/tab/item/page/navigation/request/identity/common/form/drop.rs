use super::list::{item::Value, List};
use crate::profile::Profile;
use adw::{
    prelude::{AdwDialogExt, AlertDialogExt, AlertDialogExtManual},
    AlertDialog, ResponseAppearance,
};
use gtk::{
    glib::timeout_add_seconds_local_once,
    prelude::{ButtonExt, WidgetExt},
    Button,
};
use std::rc::Rc;

// Defaults

const LABEL: &str = "Delete";
const TOOLTIP_TEXT: &str = "Drop selected identity from profile";
const MARGIN: i32 = 8;

const HEADING: &str = "Delete";
const BODY: &str = "Delete selected identity from profile?";
const RESPONSE_CANCEL: (&str, &str) = ("cancel", "Cancel");
const RESPONSE_CONFIRM: (&str, &str) = ("confirm", "Confirm");

pub struct Drop {
    pub button: Button,
}

impl Drop {
    // Constructors

    /// Create new `Self`
    pub fn build(profile: &Rc<Profile>, list: &Rc<List>) -> Self {
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
            move |_| {
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
                            move |_, _| match profile.identity.delete(profile_identity_id) {
                                Ok(_) => {
                                    if list.remove(profile_identity_id).is_some() {
                                        button.set_css_classes(&["success"]);
                                        button.set_label("Identity successfully deleted")
                                    } else {
                                        button.set_css_classes(&["error"]);
                                        button.set_label("List item not found")
                                    }
                                    timeout_add_seconds_local_once(1, {
                                        let button = button.clone();
                                        move || {
                                            button.remove_css_class("error");
                                            button.remove_css_class("success");
                                            button.set_label(LABEL)
                                        }
                                    });
                                }
                                Err(e) => {
                                    button.set_css_classes(&["error"]);
                                    button.set_label(&e.to_string())
                                }
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

    pub fn update(&self, is_visible: bool) {
        self.button.set_visible(is_visible)
    }
}
