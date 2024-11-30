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
    pub gobject: AlertDialog,
}

impl Widget {
    // Constructors

    /// Create new `Self`
    pub fn new(profile: Rc<Profile>) -> Self {
        // Init actions
        let action = Rc::new(Action::new());

        // Init child container
        let form = Rc::new(Form::new(profile, action.clone()));

        // Init main `GObject`
        let gobject = AlertDialog::builder()
            .heading(HEADING)
            .body(BODY)
            .close_response(RESPONSE_CANCEL.0)
            .default_response(RESPONSE_APPLY.0)
            .extra_child(&form.gobject)
            .build();

        // Set response variants
        gobject.add_responses(&[
            RESPONSE_CANCEL,
            // RESPONSE_MANAGE,
            RESPONSE_APPLY,
        ]);

        // Deactivate not implemented feature @TODO
        // gobject.set_response_enabled(RESPONSE_MANAGE.0, false);

        // Decorate default response preset
        gobject.set_response_appearance(RESPONSE_APPLY.0, ResponseAppearance::Suggested);
        gobject.set_response_appearance(RESPONSE_CANCEL.0, ResponseAppearance::Destructive);

        // Init events
        action.update.connect_activate({
            let form = form.clone();
            let gobject = gobject.clone();
            move || {
                // Deactivate apply button if the form values could not be processed
                gobject.set_response_enabled(RESPONSE_APPLY.0, form.is_valid());
            }
        });

        // Return new activated `Self`
        Self {
            // action,
            form,
            gobject,
        }
    }

    // Actions

    /// Callback wrapper for `apply` [response](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/signal.AlertDialog.response.html)
    /// * return `Value` enum or new record request on `None`
    pub fn on_apply(&self, callback: impl Fn(Value) + 'static) {
        self.gobject.connect_response(Some(RESPONSE_APPLY.0), {
            let form = self.form.clone();
            move |this, response| {
                // Prevent double-click action
                this.set_response_enabled(response, false);

                // Result
                callback(form.list.selected())
            }
        });
    }

    /// Show dialog with new preset
    pub fn present(&self, parent: Option<&impl IsA<gtk::Widget>>) {
        self.gobject.present(parent)
    }
}
