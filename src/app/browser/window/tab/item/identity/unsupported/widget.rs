use adw::{
    prelude::{AdwDialogExt, AlertDialogExt, AlertDialogExtManual},
    AlertDialog,
};
use gtk::prelude::IsA;

const HEADING: &str = "Oops";
const BODY: &str = "Identity not supported for this request";
const RESPONSE_QUIT: (&str, &str) = ("close", "Close");

pub struct Widget {
    gobject: AlertDialog,
}

impl Default for Widget {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        // Init gobject
        let gobject = AlertDialog::builder()
            .heading(HEADING)
            .body(BODY)
            .close_response(RESPONSE_QUIT.0)
            .default_response(RESPONSE_QUIT.0)
            .build();

        // Set response variants
        gobject.add_responses(&[RESPONSE_QUIT]);

        // Decorate default response preset
        /* contrast issue with Ubuntu orange accents
        gobject.set_response_appearance(RESPONSE_QUIT.0, ResponseAppearance::Destructive); */

        // Init events
        gobject.connect_response(None, move |dialog, response| {
            if response == RESPONSE_QUIT.0 {
                dialog.close();
            }
        });

        // Return new activated `Self`
        Self { gobject }
    }

    // Actions

    /// Show dialog for given parent
    pub fn present(&self, parent: Option<&impl IsA<gtk::Widget>>) {
        self.gobject.present(parent)
    }
}
