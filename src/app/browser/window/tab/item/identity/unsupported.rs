use adw::{
    prelude::{AdwDialogExt, AlertDialogExt, AlertDialogExtManual},
    AlertDialog,
};

const HEADING: &str = "Oops";
const BODY: &str = "Identity not supported for this request";
const RESPONSE_QUIT: (&str, &str) = ("close", "Close");

pub trait Unsupported {
    fn unsupported() -> Self;
}

impl Unsupported for AlertDialog {
    // Construct

    /// Create new `Self`
    fn unsupported() -> Self {
        // Init gobject
        let this = AlertDialog::builder()
            .heading(HEADING)
            .body(BODY)
            .close_response(RESPONSE_QUIT.0)
            .default_response(RESPONSE_QUIT.0)
            .build();

        // Set response variants
        this.add_responses(&[RESPONSE_QUIT]);

        // Decorate default response preset
        /* contrast issue with Ubuntu orange accents
        this.set_response_appearance(RESPONSE_QUIT.0, ResponseAppearance::Destructive); */

        // Init events
        this.connect_response(None, move |dialog, response| {
            if response == RESPONSE_QUIT.0 {
                dialog.close();
            }
        });

        // Return new activated `Self`
        this
    }
}
