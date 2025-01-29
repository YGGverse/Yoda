use adw::AlertDialog;

pub trait Unsupported {
    fn unsupported() -> Self;
}

impl Unsupported for AlertDialog {
    // Construct

    /// Create new `Self`
    fn unsupported() -> Self {
        use adw::prelude::{AdwDialogExt, AlertDialogExt, AlertDialogExtManual};

        const HEADING: &str = "Oops";
        const BODY: &str = "Identity not supported for this request";
        const RESPONSE_QUIT: (&str, &str) = ("close", "Close");

        // Init gobject
        let alert_dialog = AlertDialog::builder()
            .heading(HEADING)
            .body(BODY)
            .close_response(RESPONSE_QUIT.0)
            .default_response(RESPONSE_QUIT.0)
            .build();

        // Set response variants
        alert_dialog.add_responses(&[RESPONSE_QUIT]);

        // Decorate default response preset
        /* contrast issue with Ubuntu orange accents
        alert_dialog.set_response_appearance(RESPONSE_QUIT.0, ResponseAppearance::Destructive); */

        // Init events
        alert_dialog.connect_response(None, move |dialog, response| {
            if response == RESPONSE_QUIT.0 {
                dialog.close();
            }
        });

        // Return new activated `Self`
        alert_dialog
    }
}
