mod form;

use form::Form;

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
    pub form: Rc<Form>,
    pub gobject: AlertDialog,
}

impl Widget {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        // Init child container
        let form = Rc::new(Form::new());

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

        // Return new activated `Self`
        Self { form, gobject }
    }

    // Actions

    /// Callback wrapper for `apply` [response](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/signal.AlertDialog.response.html)
    /// * return `profile_identity_gemini_id` or new record request on `None`
    pub fn on_apply(&self, callback: impl Fn(Option<i64>) + 'static) {
        self.gobject.connect_response(Some(RESPONSE_APPLY.0), {
            let form = self.form.clone();
            move |_, _| callback(form.list.selected())
        });
    }

    /// Show dialog with new preset
    pub fn present(&self, parent: Option<&impl IsA<gtk::Widget>>) {
        self.gobject.present(parent)
    }
}
