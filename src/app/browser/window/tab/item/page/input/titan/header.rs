mod form;

use gtk::{glib::GString, prelude::IsA, Widget};

#[derive(Default, Clone)]
pub struct Header {
    pub mime: Option<GString>,
    pub token: Option<GString>,
}

impl Header {
    /// Show header options dialog for the referrer `widget`
    /// * takes ownership of `Self`, return new updated copy in `callback` function
    pub fn dialog(self, widget: Option<&impl IsA<Widget>>, callback: impl Fn(Self) + 'static) {
        use adw::{
            prelude::{AdwDialogExt, AlertDialogExt, AlertDialogExtManual},
            AlertDialog, ResponseAppearance,
        };
        use form::Form;

        // Response variants
        const RESPONSE_APPLY: (&str, &str) = ("apply", "Apply");
        const RESPONSE_CANCEL: (&str, &str) = ("cancel", "Cancel");

        // Init form components
        let form = Form::build(
            &self.mime.clone().unwrap_or_default(),
            &self.token.clone().unwrap_or_default(),
        );

        // Init main widget
        let alert_dialog = AlertDialog::builder()
            .heading("Options")
            .body("Customize response header")
            .close_response(RESPONSE_CANCEL.0)
            .default_response(RESPONSE_APPLY.0)
            .extra_child(&form.g_box)
            .build();

        alert_dialog.add_responses(&[RESPONSE_CANCEL, RESPONSE_APPLY]);

        // Decorate default response preset
        alert_dialog.set_response_appearance(RESPONSE_APPLY.0, ResponseAppearance::Suggested);
        /* contrast issue with Ubuntu orange accents
        alert_dialog.set_response_appearance(RESPONSE_CANCEL.0, ResponseAppearance::Destructive); */

        // Init events

        alert_dialog.connect_response(None, move |this, response| {
            this.set_response_enabled(response, false); // prevent double-click
            callback(if response == RESPONSE_APPLY.0 {
                Self {
                    mime: form.mime(),
                    token: form.token(),
                }
            } else {
                self.clone()
            })
        });

        // Show
        alert_dialog.present(widget);
    }
}
