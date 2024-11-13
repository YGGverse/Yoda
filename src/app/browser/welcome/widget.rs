use adw::{
    prelude::{AdwDialogExt, AlertDialogExtManual},
    AlertDialog,
};
use gtk::prelude::IsA;

const HEADING: &str = "Welcome!";
const BODY: &str = "Select profile for browser data";
const RESPONSE_QUIT: (&str, &str) = ("quit", "Quit");
const RESPONSE_CREATE: (&str, &str) = ("create", "Create new profile");

pub struct Widget {
    gobject: AlertDialog,
}

impl Widget {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        // Init gobject
        let gobject = AlertDialog::builder()
            .heading(HEADING)
            .body(BODY)
            .default_response(RESPONSE_QUIT.1)
            .build();

        gobject.add_responses(&[RESPONSE_QUIT, RESPONSE_CREATE]);

        // Return new `Self`
        Self { gobject }
    }

    // Actions

    /// Show dialog for parent [Window](https://docs.gtk.org/gtk4/class.Window.html)
    pub fn present(&self, parent: Option<&impl IsA<gtk::Widget>>) {
        self.gobject.present(parent);
    }

    /* @TODO not in use
    /// Get reference to GObject
    ///
    pub fn gobject(&self) -> &AlertDialog {
        &self.gobject
    } */
}
