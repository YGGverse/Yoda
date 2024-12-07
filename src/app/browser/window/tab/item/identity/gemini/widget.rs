mod action;
pub mod form;

use action::Action;
use form::{list::item::value::Value, Form};

use crate::profile::Profile;
use adw::{
    prelude::{AdwDialogExt, AlertDialogExt, AlertDialogExtManual},
    AlertDialog, ResponseAppearance,
};
use gtk::prelude::IsA;
use std::rc::Rc;

// Defaults
const HEADING: &str = "Ident";
const BODY: &str = "Select identity certificate";

// Response variants
const RESPONSE_APPLY: (&str, &str) = ("apply", "Apply");
const RESPONSE_CANCEL: (&str, &str) = ("cancel", "Cancel");
// const RESPONSE_MANAGE: (&str, &str) = ("manage", "Manage");

// Select options

pub struct Widget {
    // pub action: Rc<Action>,
    pub form: Rc<Form>,
    pub alert_dialog: AlertDialog,
}

impl Widget {
    // Constructors

    /// Create new `Self`
    pub fn new(profile: Rc<Profile>, auth_url: &str) -> Self {
        // Init actions
        let action = Rc::new(Action::new());

        // Init child container
        let form = Rc::new(Form::new(profile, action.clone(), auth_url));

        // Init main widget
        let alert_dialog = AlertDialog::builder()
            .heading(HEADING)
            .body(BODY)
            .close_response(RESPONSE_CANCEL.0)
            .default_response(RESPONSE_APPLY.0)
            .extra_child(&form.g_box)
            .build();

        // Set response variants
        alert_dialog.add_responses(&[
            RESPONSE_CANCEL,
            // RESPONSE_MANAGE,
            RESPONSE_APPLY,
        ]);

        // Deactivate not implemented feature @TODO
        // alert_dialog.set_response_enabled(RESPONSE_MANAGE.0, false);

        // Decorate default response preset
        alert_dialog.set_response_appearance(RESPONSE_APPLY.0, ResponseAppearance::Suggested);
        alert_dialog.set_response_appearance(RESPONSE_CANCEL.0, ResponseAppearance::Destructive);

        // Init events
        action.update.connect_activate({
            let form = form.clone();
            let alert_dialog = alert_dialog.clone();
            move || {
                // Update child components
                form.update();

                // Deactivate apply button if the form values could not be processed
                alert_dialog.set_response_enabled(RESPONSE_APPLY.0, form.is_applicable());
            }
        });

        // Return new activated `Self`
        Self {
            // action,
            form,
            alert_dialog,
        }
    }

    // Actions

    /// Callback wrapper for `apply` [response](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/signal.AlertDialog.response.html)
    /// * return `Value` enum or new record request on `None`
    pub fn on_apply(&self, callback: impl Fn(Value) + 'static) {
        self.alert_dialog.connect_response(Some(RESPONSE_APPLY.0), {
            let form = self.form.clone();
            move |this, response| {
                // Prevent double-click action
                this.set_response_enabled(response, false);

                // Result
                callback(form.list.selected().value_enum())
            }
        });
    }

    /// Show dialog with new preset
    pub fn present(&self, parent: Option<&impl IsA<gtk::Widget>>) {
        self.form.update();
        self.alert_dialog.present(parent)
    }
}
