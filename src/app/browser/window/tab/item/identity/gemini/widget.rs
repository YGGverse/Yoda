mod form;

use form::Form;

use adw::{
    prelude::{AdwDialogExt, AlertDialogExt, AlertDialogExtManual},
    AlertDialog, ResponseAppearance,
};
use gtk::prelude::IsA;

// Defaults
const HEADING: &str = "Ident";
const BODY: &str = "Select identity certificate";

// Response variants
const RESPONSE_APPLY: (&str, &str) = ("apply", "Apply");
const RESPONSE_CANCEL: (&str, &str) = ("cancel", "Cancel");
// const RESPONSE_MANAGE: (&str, &str) = ("manage", "Manage");

// List options
const OPTION_CREATE: (Option<i64>, &str) = (None, "Create new..");

// Select options

pub struct Widget {
    gobject: AlertDialog,
}

impl Widget {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        // Collect identity certificates
        let mut options: Vec<(Option<i64>, String, bool)> = Vec::new();
        options.push((OPTION_CREATE.0, OPTION_CREATE.1.to_owned(), false));

        // Init child container
        let form = Form::new(options);

        // Init main `GObject`
        let gobject = AlertDialog::builder()
            .heading(HEADING)
            .body(BODY)
            .close_response(RESPONSE_CANCEL.0)
            .default_response(RESPONSE_APPLY.0)
            .extra_child(form.gobject())
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
        Self { gobject }
    }

    // Actions

    /// Wrapper for default [response](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/signal.AlertDialog.response.html) signal
    /// * return `profile_identity_gemini_id` or new record request on `None`
    pub fn connect_response(&self, callback: impl Fn(Option<i64>) + 'static) {
        self.gobject.connect_response(None, move |_, response| {
            if response == RESPONSE_APPLY.0 {
                callback(None)
            } else {
                callback(None)
            } // @TODO
        });
    }

    /// Show dialog with new preset
    pub fn present(&self, parent: Option<&impl IsA<gtk::Widget>>) {
        self.gobject.present(parent)
    }
}
